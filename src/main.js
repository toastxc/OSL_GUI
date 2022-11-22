const { invoke } = window.__TAURI__.tauri;







// elements for OSL commands
const connect = document.getElementById("connect");

const get = document.getElementById("get");

const products = document.getElementById("products");

// returning results to screen
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
	let original = get.innerHTML;

	get.innerHTML = "..";

	let stat = await invoke('get');

	get.innerHTML = original;

	result.innerHTML = stat;

});



// products
products.addEventListener('click', async function products_click() {
        let original = get.innerHTML;

        get.innerHTML = "..";

        let stat = await invoke('products');

        get.innerHTML = original;

        result.innerHTML = stat;

});

