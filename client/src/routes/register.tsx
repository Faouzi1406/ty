import { createSignal } from "solid-js";

type Register =  {
  username: string;
  password: string;
  email: string;
}

export default function Register() {
  const [form, setForm] = createSignal<Register>({ username: "", password: "", email: "" });
  const [formError, setFormError] = createSignal<Register>({ username: "", password: "", email: "" });

  const handleInput = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setForm({ ...form(), [target.name]: target.value });
  };

  const sendForm = async () => {
    const res = await fetch("http://localhost:8080/users/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(form()),
    });
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

    if (form().email.length < 6) {
      setFormError({ ...formError(), email: "Email must be at least 6 characters" });
      return;
    } else {
      setFormError({ ...formError(), email: "" });
    }

    sendForm();
  }

  return <div class="flex justify-center bg-primary  min-h-[100vh]">
    {/* This is the login form */}
    <div class="rounded-md mt-10 md:w-3/4 lg:w-2/6 w-screen border-gray-400  h-52 p-2">
      <h1 class="text-4xl font-bold text-white">Login</h1>
      <form class="flex flex-col gap-4 mt-10">


        <label class="text-xl font-bold text-white ">Username</label>
        <input class="border border-primary h-10 bg-gray-700 p-1 text-white" name="username" onInput={(e) => handleInput(e)} />
        <span class="text-red-600">{formError().username}</span>

        <label class="text-xl font-bold text-white rounded">Password</label>
        <input class="border border-primary h-10 bg-gray-700 p-1 text-white" name="password" onInput={(e) => handleInput(e)} type="password" />
        <span class="text-red-600">{formError().password}</span>

        <label class="text-xl font-bold text-white rounded">Email</label>
        <input class="border border-primary h-10 bg-gray-700 p-1 text-white" name="email" onInput={(e) => handleInput(e)} type="text" />
        <span class="text-red-600">{formError().email}</span>

        <button class="font-bold text-lg text-white h-10 bg-secondary" onClick={handleSubmit} >Create</button>
      </form>
      <div class="flex gap-2 mt-4">
        <span class="text-white">Already have and account?</span>
        <a href="/login" class="text-blue-500">Login</a>
      </div>
    </div>
  </div>;

}
