function handleToast(type, message, timer = 8000) {
  let toast = document.getElementById("toast");
  let toastMessage = document.getElementById("toast_message");
  toast.classList.add("slideInDown");
  toast.classList.add(type);

  toastMessage.innerText = message;

  setTimeout(() => {
    toast.classList.remove("slideInDown");
    toast.classList.remove(type);
  }, timer);
}
