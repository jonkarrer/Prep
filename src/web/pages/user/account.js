import { createToast } from "/pages/global/components/Toast.js";

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

let changeEmailForm = document.getElementById("change_email_form");
let changePasswordForm = document.getElementById("change_password_form");
let deleteAccountForm = document.getElementById("delete_account_form");

changeEmailForm.onsubmit = (e) => submitForm(e);
changePasswordForm.onsubmit = (e) => submitForm(e);
deleteAccountForm.onsubmit = (e) => submitForm(e);
