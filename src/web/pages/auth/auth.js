import { createToast } from "/pages/global/components/Toast.js";

const loginForm = document.getElementById("login_form");
const registerForm = document.getElementById("register_form");
const signInLink = document.getElementById("sign_in_link");
const signUpLink = document.getElementById("sign_up_link");

loginForm.addEventListener("submit", submitForm);
registerForm.addEventListener("submit", submitRegisterForm);
signInLink.addEventListener("click", showLoginForm);
signUpLink.addEventListener("click", showRegisterForm);

async function submitRegisterForm(event) {
  event.preventDefault();

  const formData = new FormData(event.target);

  let password = formData.get("password");
  let confirmPassword = formData.get("confirm_password");

  if (password != confirmPassword) {
    createToast("error", "Passwords Do Not Match");
    return;
  }

  const path = event.target.action;
  const method = event.target.method;
  const body = new URLSearchParams(formData).toString();

  let res = await fetch(path, {
    method: method,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    redirect: "follow",
    body: body,
  });

  if (res.redirected) {
    window.location = res.url;
  } else {
    let text = await res.text();
    createToast("error", text);
  }
}

async function submitForm(event) {
  event.preventDefault();

  const formData = new FormData(event.target);
  const path = event.target.action;
  const method = event.target.method;
  const body = new URLSearchParams(formData).toString();

  let res = await fetch(path, {
    method: method,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    redirect: "follow",
    body: body,
  });

  if (res.redirected) {
    window.location = res.url;
  } else {
    let text = await res.text();
    createToast("error", text);
  }
}

function registerFormAnimation() {}

function showRegisterForm() {
  signInLink.classList.add("show");
  registerForm.classList.add("show");

  signUpLink.classList.remove("show");
  loginForm.classList.remove("show");
}

function showLoginForm() {
  signUpLink.classList.add("show");
  loginForm.classList.add("show");

  signInLink.classList.remove("show");
  registerForm.classList.remove("show");
}

function transitionSliderText(index) {
  let textEl = document.getElementById("slider_text");
  textEl.classList.add("fade-in");

  textEl.innerText = [
    "Organize your kitchen with Prep, the generative recipe system.",
    "Don't let a great dish be forgotten, let Prep save it for next time.",
    "Prep will help you learn, understand, and execute any recipe",
  ][index];

  setTimeout(() => textEl.classList.remove("fade-in"), 2000);
}

function transitionFilledDot(index) {
  let dot_one = document.getElementById("dot_one");
  let dot_two = document.getElementById("dot_two");
  let dot_three = document.getElementById("dot_three");

  let dots = [dot_one, dot_two, dot_three];

  let currDot = dots[index];
  let prevDot = dots[index - 1 === -1 ? 2 : index - 1];

  currDot.classList.toggle("Dot--filled");
  currDot.classList.add("fill-in");

  prevDot.classList.toggle("Dot--filled");

  setTimeout(() => currDot.classList.remove("fill-in"), 2000);
}

let i = 1;
setInterval(() => {
  transitionSliderText(i);
  transitionFilledDot(i);

  i = i === 2 ? 0 : (i += 1);
}, 5000);
