// @refresh reload
import { Suspense } from "solid-js";
import {
  A,
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Meta,
  Routes,
  Scripts,
  Title,
} from "solid-start";
import "./root.css";

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>SolidStart - With TailwindCSS</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <div>
              <nav class="bg-gray-800">
                <ul class="flex gap-2 text-gray-200 h-12 items-center px-5">
                  <div class="flex-grow">
                    <A href="/" class="text-2xl font-semibold">Ty</A>
                  </div>
                  <div class="">
                    <A href="/login" class="text-lg font-semibold">Login</A>
                  </div>
                </ul>
              </nav>
            </div>
            <Routes>
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
