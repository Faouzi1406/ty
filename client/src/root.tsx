// @refresh reload
import { Suspense } from "solid-js";
import Auth from "./authentication/auth";
import { createSignal } from "solid-js";
import { User } from "./authentication/auth";
import {
  A,
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Meta,
  redirect,
  Route,
  RouteDataArgs,
  Routes,
  Scripts,
  Title,
} from "solid-start";
import "./root.css";
import WatchVideo from "./routes/video/[id]";
import Dropdown from "./components/Dropdown";


const videoData = async ({ params, location, navigate, data }: RouteDataArgs) => {
  const video = await fetch(`http://localhost:8080/videos/select/watch/${params.id}`);
  const videoFile = await video.blob()

  const fetchVidoeInfo = await fetch(`http://localhost:8080/videos/select/${params.id}`);
  const videoInfo = await fetchVidoeInfo.json();

  return [videoFile, videoInfo];
}

export default function Root() {
  const [user, setUser] = createSignal<User | undefined>();
  const auth = new Auth;

  const getUser = async () => {
    const user = await auth.getUser();
    typeof user.payload != 'string' ? setUser(user.payload) : setUser(undefined);
  }
  getUser();


  return (
    <Html lang="en">
      <Head>
        <Title>TY - Share your video's</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <nav class="top-0 sticky bg-primary shadow-lg py-2 px-2">
              <ul class="flex gap-2 text-gray-200 h-12 items-center px-5">
                <div class="flex-grow">
                  <A href="/" class="text-2xl font-semibold">Ty</A>
                </div>
                <div class="flex items-center gap-3 ">
                  {user() ?
                    <div class="flex items-center gap-3">
                      <Dropdown props={user()} />
                    </div>
                    :
                    <A href="/register" class="text-lg font-semibold">Login</A>}
                </div>
              </ul>
            </nav>
            <Routes>
              <Route
                path={"/video/:id"}
                component={WatchVideo}
                data={videoData}
              />
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
