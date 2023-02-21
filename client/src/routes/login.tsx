import { createSignal } from "solid-js";
import { redirect } from "solid-start";
import server$ from "solid-start/server";
import Auth from "~/authentication/auth";


type Form = {
  username: string;
  password: string;
};

export default function Login() {
  const [form, setForm] = createSignal<Form>({ username: "", password: "" });
  const [formError, setFormError] = createSignal<Form>({ username: "", password: "" });
  const [loginError, setLoginError] = createSignal<String | undefined>();
  const [user, setUser] = createSignal<User | undefined>();
  const auth = new Auth;

  const getUser = async () => {
    const user = await auth.getUser();
    console.log(user);

    if (user.type != 'error') {
      window.location.href = '/';
    }
  }

  getUser();



  const handleInput = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setForm({ ...form(), [target.name]: target.value });
  };

  const sendForm = async () => {
    const res = await fetch("http://localhost:8080/users/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(form()),
    });

    if (!res.ok) {
      setLoginError("Invalid username or password");
      return;
    }

    setLoginError(undefined);

    const data = await res.text();
    const session = new Auth();
    session.createSession(data);
  }


  const handleSubmit = (e: Event) => {
    e.preventDefault();
    console.log(form());

    if (form().username.length < 6) {
      setFormError({ ...formError(), username: "Username must be at least 6 characters" });
      return;
    } else {
      setFormError({ ...formError(), username: "" });
    }

    if (form().password.length < 6) {
      setFormError({ ...formError(), password: "Password must be at least 6 characters" });
      return;
    } else {
      setFormError({ ...formError(), password: "" });
    }

    sendForm();

  }

  return <div class="flex justify-center bg-primary  min-h-[100vh]">
    {/* This is the login form */}
    <div class="rounded-md mt-10 md:w-3/4 lg:w-2/6 w-screen border-gray-400  h-52 p-2">
      <h1 class="text-4xl font-bold text-white">Login</h1>
      {
        loginError() != undefined ? <div class="text-red-200 bg-red-600 w-2/3 px-2 py-3 rounded mt-2">
          {loginError()}
        </div> : <></>
      }


      <form class="flex flex-col gap-4 mt-10">
        <label class="text-xl font-bold text-white ">Username</label>
        <input class="border border-primary h-10 bg-gray-700 p-1 text-white" name="username" onInput={(e) => handleInput(e)} />
        <span class="text-red-600">{formError().username}</span>

        <label class="text-xl font-bold text-white rounded">Password</label>
        <input class="border border-primary h-10 bg-gray-700 p-1 text-white" name="password" onInput={(e) => handleInput(e)} type="password" />
        <span class="text-red-600">{formError().password}</span>
        {/* This is the error message if username.length <6 */}
        <span class="hidden">
          <span class="text-red-500">Username must be at least 6 characters</span>
        </span>

        <button class="font-bold text-lg text-white h-10 bg-secondary" onClick={handleSubmit} >Login</button>
      </form>
      <div class="flex gap-2 mt-4">
        <span class="text-white">Don't have an account?</span>
        <a href="/register" class="text-blue-500">Register</a>
      </div>
    </div>
  </div>;
}
