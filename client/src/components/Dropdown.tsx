import { createSignal, mergeProps } from "solid-js";
import { User } from "~/authentication/auth";
import { Show } from "~/types/Dropdown";

const DropdownInfo = ({ user }: { user: User | undefined }) => {

  return <div class="absolute top-10  border w-52 right-16 rounded-md bg-primary shadow-md p-2">
    <div class="flex items-center">
      <img src={user?.profile_pic.toString() || ''} class="rounded w-10 h-10" />
      <p class="text-blue-200">@{user?.username.toString() || ''}</p>
    </div>
    <div class="flex items-center mt-5">
      <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" fill="currentColor" class="bi bi-plus" viewBox="0 0 16 16">
        <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z" />
      </svg>
      <a class="font-semibold" href="/create_video">Upload video</a>
    </div>
  </div>
}

export default function Dropdown({ props }: { props: User | undefined }) {
  const [display, setDisplay] = createSignal<Show>('hide');

  const handleDropDown = () => {
    if (display() == 'hide') {
      setDisplay('show')
    } else {
      setDisplay('hide')
    }
  };

  return <div class="" onClick={() => handleDropDown()}>
    <img class="rounded-full w-12 aspect-square border p-1" src={props?.profile_pic.toString() || ''} />
    {display() == 'show' ? <DropdownInfo user={props} /> : <></>}
  </div>;
}


