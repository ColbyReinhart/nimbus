document.querySelector("#formSubmit").addEventListener("click", () => {
	console.log("test");
	let uploadForm = document.querySelector("#uploadResource");
	uploadForm.action = document.querySelector("#uploadFile").files[0].name;
	console.log(uploadForm.action);
	uploadForm.submit();
});