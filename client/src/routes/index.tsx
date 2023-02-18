import { A } from "solid-start";
import Counter from "~/components/Counter";
import { createSignal } from "solid-js";

const uploadFile = (ws: WebSocket | undefined, file: File) => {
  let chunkSize = 16384;
  let offset = 0;
  let slice = file.slice(offset, offset + chunkSize);
  let reader = new FileReader();
  reader.readAsArrayBuffer(slice);
  const testName = 'test.mp4';

  if (ws != undefined) {
    ws.send(JSON.stringify({file_name: testName, file_size: file.size}))
    reader.onload = (e) => {
      if (e.target?.result) {
        console.log("Sending chunk");
        ws.send(e.target.result);
        offset += chunkSize;
        if (offset < file.size) {
          slice = file.slice(offset, offset + chunkSize);
          reader.readAsArrayBuffer(slice);
        }
      }
    }
  }
}

const Upload = () => {
  const [file, setFile] = createSignal<File | undefined>();

  const [webSocket, setWebSocket] = createSignal<WebSocket | undefined>();

  if (typeof window !== 'undefined') {
    const websocket = new WebSocket('ws://localhost:8080/videos/sockets/upload');
    websocket.onopen = () => {
    }

    websocket.onmessage = (event) => {
      console.log(event.data);
    }

    websocket.onclose = () => {
      console.log('Websocket disconnected');
    }

    setWebSocket(websocket);
  }



  const handleFile = (e: Event) => {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file?.type !== 'video/mp4') {
      e.preventDefault();
      return;
    };

    setFile(file);
  }

  const handleUpload = () => {
    if (file()) {
      uploadFile(webSocket(), file());
    }
    console.log('Upload');
  }

  console.log(file());

  return (
    <div >
      <input type="file" onChange={e => handleFile(e)} />
      <div>
        {file() && <p>{file()?.name}</p>}
      </div>
      <button onClick={handleUpload}>Upload</button>
    </div>
  )
}

export default function Home() {
  return (
    <main class="">
      <Upload />
    </main>
  );
}
