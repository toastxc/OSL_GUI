const { invoke } = window.__TAURI__.tauri;




// elements for OSL commands
const connect = document.getElementById("connect");

const get = document.getElementById("get");

const products = document.getElementById("products");

const info = document.getElementById("info");

const install = document.getElementById("install");



// user input
const input = document.getElementById("input");

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
        let original = products.innerHTML;

        products.innerHTML = "..";

        let stat = await invoke('products');

        products.innerHTML = original;

        result.innerHTML = stat;

});






info.addEventListener('click', async function info_click() {
        
	let original = info.innerHTML;

        info.innerHTML = "..";

	let va = input.value;

	let stat = await invoke('info', { products: va });

        info.innerHTML = original;

        result.innerHTML = stat;

});




redeem.addEventListener('click', async function redeem_click() {

        let original = redeem.innerHTML;

        redeem.innerHTML = "..";

        let va = input.value;

        let stat = await invoke('redeem', { key: va });

        redeem.innerHTML = original;

        result.innerHTML = stat;

});









