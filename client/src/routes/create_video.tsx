import { createSignal } from "solid-js";
import Auth, { User } from "~/authentication/auth";

type FormVideo = {
  title: string,
  description: string,
  user_id: number,
  file_size: number,
  thumb_mail_url: String | undefined
}

type Loading = 'isLoading' | 'Done' | 'None';

export default function CreateVideo() {
  const [user, setUser] = createSignal<User>();
  const [video, setVideo] = createSignal<File>();
  const [formError, hanldeFormError] = createSignal<FormVideo>({ title: '', description: '', user_id: 0, file_size: 0, thumb_mail_url: undefined });
  const [form, setForm] = createSignal<FormVideo>({ title: '', description: '', user_id: 0, file_size: 0, thumb_mail_url: undefined });
  const [uploadError, setUploadError] = createSignal<string | undefined>();
  const [websocket, setWebsocket] = createSignal<WebSocket | undefined>();
  const [uploading, setUploading] = createSignal<Loading>('None');


  const getUser = async () => {
    const auth = new Auth();
    const user = await auth.getUser();

    if (user.type != 'error') {
      //@ts-ignore
      setUser(user.payload);
      //@ts-ignore
      setForm({ ...form(), user_id: user.payload.id });
    } else {
      window.location.href = '/login';
    }
  }

  const handleForm = (e: Event) => {
    const target = e.target as HTMLInputElement;

    if (target.name == 'title' && target.value.length < 10 || target.value.length > 50) {
      console.log('hello')
      hanldeFormError({ ...formError(), title: 'Title must have more than 10 characters & must be less than 50 characters' });
    } else {
      hanldeFormError({ ...formError(), title: '' });
    }

    if (target.name == 'description' && target.value.length < 10 || target.value.length > 50) {
      console.log("wow");
      hanldeFormError({ ...formError(), description: 'Description must have more than 10 characters & must be less than 50 characters' });
    } else {
      hanldeFormError({ ...formError(), description: '' });
    }

    console.log(form());
    setForm({ ...form(), [target.name]: target.value });
  }

  const handleImage = (e: InputEvent) => {
    const target = e.target as HTMLInputElement;
    if (!target.files) return;
    const file = target.files[0];

    if (file.type != 'image/png') {
      e.preventDefault();
    }

    const fileReader = new FileReader();

    fileReader.readAsDataURL(file);

    fileReader.onload = (e) => {
      //@ts-ignore
      setForm({ ...form(), thumb_mail_url: fileReader.result.split(',')[1]  as String });
    }
  };

  const handleVideo = (e: InputEvent) => {
    const target = e.target as HTMLInputElement;
    if (!target.files) return;
    const file = target.files[0];

    if (file.type != 'video/mp4') {
      e.preventDefault();
    }

    setVideo(file);
  }

  const handleVidoeSubmit = async (e: Event) => {
    e.preventDefault();
    if (formError().title != '' || formError().description != '') {
      setUploadError("Please fix the errors");
      return;
    }


    if (video() == undefined) {
      console.log('hello');
      setUploadError("Pleas select a video before uploading");
      return;
    } else {
      setUploadError("");
    };

    const websocket = new WebSocket('ws://localhost:8080/videos/sockets/upload');

    websocket.onopen = () => {


      setForm({ ...form(), file_size: video()!.size });
      websocket.send(JSON.stringify(form()));

      const reader = new FileReader();


      const file = video() as File;
      const chunkSize = 16384;
      let offset = 0;
      let slice = file.slice(offset, offset + chunkSize);
      reader.readAsArrayBuffer(slice);

      reader.onload = (e) => {
        if (e.target?.result) {
          websocket.send(e.target.result);
          offset += chunkSize;
          if (offset < file.size) {
            slice = file.slice(offset, offset + chunkSize);
            reader.readAsArrayBuffer(slice);
          }
        }
      }
    }

    websocket.onmessage = (e) => {
      if (e.data == "uploadd") {
        window.location.href = "/";
      }
    }

    setUploading('isLoading');
    console.log("fast?");
  };


  getUser();

  return (
    <div class="bg-primary min-h-screen">
      <form class="flex flex-col items-center justify-center h-full ">
        <h1 class="text-3xl text-white">Create Video</h1>
        <span class="text-red-600">{uploadError()}</span>
        <div class="w-full md:w-2/3 lg:w-1/3 ">
          <label class="text-white font-bold text-lg">Title</label>
          <input type="text" name="title" class="w-full h-10  mt-2 mb-2 bg-secondary text-white border p-2" placeholder="Title" onInput={(e) => handleForm(e)} />
          <div>
            <span class="text-red-600 ">{formError().title}</span>
          </div>

          <label class="text-white font-bold text-lg">Description</label>
          <input type="text" name="description" class="w-full h-10 mt-2 mb-2 bg-secondary border text-white p-2" placeholder="Description" onInput={(e) => handleForm(e)} />
          <div>
            <span class="text-red-600">{formError().description}</span>
          </div>

          <label class="text-white font-bold text-lg">Video</label>
          <input
            type="file"
            name="video"
            class="w-full mt-2 mb-2  h-10 text-white p-2"
            placeholder="Video"
            onChange={// @ts-ignore 
              (e) => handleVideo(e)}
          />

          <label class="text-white font-bold text-lg">Thumbmail</label>
          <input
            type="file"
            name="video"
            class="w-full mt-2 mb-2  h-10 text-white p-2"
            placeholder="Video"
            onChange={// @ts-ignore 
              (e) => handleImage(e)}
          />
        </div>
        {
          uploading() == 'None' ?
            <button class="bg-secondary text-white font-bold text-lg w-1/3 h-10 mt-4" onClick={handleVidoeSubmit}>Create</button> :
            <button class="bg-secondary text-white font-bold text-lg w-1/3 h-10 mt-4" onClick={(e) => e.preventDefault()}>Uploading...</button>
        }
      </form>
    </div>
  )
}
