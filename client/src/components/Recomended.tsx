import { createSignal, For } from "solid-js";
import { Video } from "~/types/Video";

type response = "error" | Video[];

const getVid = async (): Promise<response> => {
  const allVideos = await fetch("http://localhost:8080/videos/all");

  if (allVideos.ok) {
    return allVideos.json();
  } else {
    return 'error';
  }
}
export const RecommendedSide = () => {
  const [videos, setVideos] = createSignal<Video[]>();

  const getVideos = async () => {
    let video = await getVid();
    if (video != "error") {
      setVideos(video);
    }
  }

  getVideos();

  return (
    <div class="flex flex-col gap-2 overflow-auto max-h-96">
      {
        videos() ? videos()?.map(e => {
          if (e.title != '') {
            return <a class="rounded border text-white p-2 w-96" href={`/video/${e.id}`}>
              <h4 class="font-semibold">{e.title.toString()}</h4>
              <div>{e.description.toString().substring(0,20) + '...'}</div>
            </a>
          } else {
            return <></>
          }
        }) : <p>No videos to recommend</p>
      }
    </div>
  )
}
