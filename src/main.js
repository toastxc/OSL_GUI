const { invoke } = window.__TAURI__.tauri;








const connect = document.getElementById("connect");
const get = document.getElementById("get");
const result = document.getElementById("result");

// connect 
connect.addEventListener('click', async function connect_click() {
	
	let original = connect.innerHTML;

	connect.innerHTML = "..";

	let stat = await invoke('connect');

	connect.innerHTML = original; 

	result.innerHTML = stat;

});


// get 
get.addEventListener('click', async function get_click() {

	get.innerHTML = "..";

	let getstat = await invoke('get');

	console.log(getstat)

	get.innerHTML = getstat;
})

