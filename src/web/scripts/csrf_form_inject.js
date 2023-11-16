const CSRF_COOKIE_KEY = "x-csrf-token";

let csrf_cookie = document.cookie
  .split(";")
  .find((item) => item.includes(CSRF_COOKIE_KEY));

let csrf_token = csrf_cookie.substring((CSRF_COOKIE_KEY + "=").length);
document.getElementById(CSRF_COOKIE_KEY).value = csrf_token;
