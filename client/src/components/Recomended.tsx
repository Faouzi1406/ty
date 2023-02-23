import { createSignal } from "solid-js";
import { User } from "~/authentication/auth";
import { Video } from "~/types/Video";

type response = "error" | [Video, User][];

const getVid = async (): Promise<response> => {
  const allVideos = await fetch("http://localhost:8080/videos/all");

  if (allVideos.ok) {
    return allVideos.json();
  } else {
    return 'error';
  }
}

const toDays = (date: string): string => {
  let toDate = new Date(date);
  let today = new Date();
  let timeMsDif = toDate.getTime() - today.getTime();
  let totalDays = Math.ceil(timeMsDif / (1000 * 3600 * 24));

  if (totalDays == 0) {
    return "Today"
  }
  else {
    return totalDays.toString()
  }
}

export const RecommendedSide = () => {
  const [videos, setVideos] = createSignal<[User, Video][]>();

  const getVideos = async () => {
    let video = await getVid();
    if (video != "error") {
      setVideos(video);
    }
  }

  getVideos();

  return (
    <div class="flex flex-col gap-2 overflow-auto px-2 py-10">
      {
        videos() ? videos()?.map(e => {
          if (e[1].title != '') {
            return <a class="w-fit flex gap-2" href={`/video/${e[1].id}`}>
              <img src={`http://localhost:8080/videos/select/thumbmail/${e[1].id}`} class="h-24 w-32 rounded-md" />
              <div>
                <p class="font-semibold text-white w-96">{e[1].title.toString().substring(0, 40)}</p>
                <div class="flex items-center gap-1">
                  <img src={e[0].profile_pic.toString()} class="w-10 h-10 aspect-square" />
                  <p class="text-white">{e[0].username.toString()}</p>
                </div>
                <p class="text-gray-300 text-sm py-2 font-semibold">{toDays(e[1].created_at.toString())}</p>
              </div>
            </a>
          } else {
            return <></>
          }
        }) : <p>No videos to recommend</p>
      }
    </div>
  )
}
