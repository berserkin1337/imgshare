const uploadForm = document.getElementById("uploadForm");
const uploadedImageSection = document.getElementById("uploadedImageSection");
const message = document.getElementById("message");
function isFileImage(file) {
  const acceptedImageTypes = [
    "image/gif",
    "image/jpeg",
    "image/png",
    "image/jpg",
    "image/webp",
    "image/svg+xml",
    "image/apng",
    "image/avif",
    "image/bmp",
  ];

  return file && acceptedImageTypes.includes(file["type"]);
}

uploadForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  const formData = new FormData(uploadForm);
  const file = formData.get("img");
  console.log(file, formData);
  //check if the file is an image
  if (!isFileImage(file)) {
    message.innerHTML =
      "<span>Please upload an image file from the provided image formats</span>";
    message.style.color = "red";
    return;
  }
  let response = await fetch("/api/uploadimage", {
    method: "POST",
    body: formData,
  });
  const data = await response.json();
  if (response.ok && data.status == "success") {
    message.innerHTML = "<span>Image uploaded successfully</span>";
    message.style.color = "green";
    uploadedImageSection.innerHTML = `<img src="${data.data.url}" alt="uploaded image" />`;
  } else {
    message.innerHTML = `<span>Image upload failed : ${data.message}</span>`;
    message.style.color = "red";
  }
});
