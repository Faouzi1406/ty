import { createSignal } from 'solid-js';
import { useRouteData } from 'solid-start';
import { User } from "~/authentication/auth";
import { RecommendedSide } from '~/components/Recomended';

type VideoInfo = {
  id: number,
  title: String,
  description?: String,
  url: String,
  created_at: String
};

export default function WatchVideo() {
  const [vid, setVideo] = createSignal<String | undefined>();
  const [vidinfo, setVideoInfo] = createSignal<VideoInfo>();
  const [user, setUser] = createSignal<User>();

  const video = async () => {
    const video = await useRouteData();
    //@ts-ignore
    const objectUrl = URL.createObjectURL(video[0]);
    setVideo(objectUrl);
    //@ts-ignore
    setVideoInfo(video[1]);
    //@ts-ignore
    setUser(video[2]);
  }
  video();

  return (
    <div class='min-h-screen bg-secondary'>
      <div class='flex  md:flex-row lg:flex-row w-full'>
        <div class='flex flex-col items-center justify-center py-10 gap-2 flex-grow'>
          {
            vid() ? <video class='rounded-md  w-3/3 md:w-3/3 lg:w-3/6' controls>
              <source src={vid()?.toString()} type="video/mp4" />
              </video> : <div>
            </div>
          }
          <div class='text-white justify-start items-start  md:w-1/3 lg:w-3/6'>
            {
              vidinfo() != undefined ? <div>
                <p class="font-bold text-2xl">{vidinfo()?.title.toString() || ''}</p>
                <div class='flex items-center gap-2'>
                  <img src={user() ? user()?.profile_pic.toString() : ''} alt="profile pic" class='rounded-full w-12 h-12 aspect-square border p-1 mt-2' />
                  <p class='font-bold'>{user() ? user()?.username.toString() : ''}</p>
                </div>
                <details class='rounded  mt-2'>
                  <summary class='font-bold'>Description</summary>
                  <p>{vidinfo() ? vidinfo()?.description?.toString() : ''}</p>
                </details>
              </div> :
                <div class='border  p-5 rounded-full font-bold'>
                  <p>Ooopsss.. It seems you found a video that doesn't exist or was removed...</p>
                </div>
            }
          </div>
        </div>

        <div>
          <RecommendedSide />
        </div>
      </div>
    </div>
  )
}
