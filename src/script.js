const { invoke } = window.__TAURI__.core;
const root = document.getElementById("root");
const triangle = document.getElementById("triangle");
let cA;
let cB;
let cC;
let sA;
let sB;
let sC;
let S;
let hA;
let hB;
let hC;
let P;
let pR;
let iR;
async function submit_triangle() {
	let out = JSON.parse(
		await invoke("submit_triangle", {
			ca: parseFloat(cA.value),
			cb: parseFloat(cB.value),
			cc: parseFloat(cC.value),
			sa: parseFloat(sA.value),
			sb: parseFloat(sB.value),
			sc: parseFloat(sC.value),
			s: parseFloat(S.value),
			ha: parseFloat(hA.value),
			hb: parseFloat(hB.value),
			hc: parseFloat(hC.value),
			p: parseFloat(P.value),
			pr: parseFloat(pR.value),
			ir: parseFloat(iR.value),
		})
	);
	root.innerHTML = out.html;
	cA.value = out.cA;
	cB.value = out.cB;
	cC.value = out.cC;
	sA.value = out.sA;
	sB.value = out.sB;
	sC.value = out.sC;
	S.value = out.S;
	hA.value = out.hA;
	hB.value = out.hB;
	hC.value = out.hC;
	P.value = out.P;
	pR.value = out.pR;
	iR.value = out.iR;
	MathJax.typeset();
}
async function new_triangle() {
	let out = JSON.parse(await invoke("new_triangle", {}));
	root.innerHTML = out.html;
	cA.value = out.cA;
	cB.value = out.cB;
	cC.value = out.cC;
	sA.value = out.sA;
	sB.value = out.sB;
	sC.value = out.sC;
	S.value = out.S;
	hA.value = out.hA;
	hB.value = out.hB;
	hC.value = out.hC;
	P.value = out.P;
	pR.value = out.pR;
	iR.value = out.iR;
	MathJax.typeset();
}
window.addEventListener("DOMContentLoaded", () => {
	cA = document.getElementById("cA");
	cB = document.getElementById("cB");
	cC = document.getElementById("cC");
	sA = document.getElementById("sA");
	sB = document.getElementById("sB");
	sC = document.getElementById("sC");
	S = document.getElementById("S");
	hA = document.getElementById("hA");
	hB = document.getElementById("hB");
	hC = document.getElementById("hC");
	P = document.getElementById("P");
	pR = document.getElementById("pR");
	iR = document.getElementById("iR");
	new_triangle();
	triangle.addEventListener("submit", (e) => {
		e.preventDefault();
		submit_triangle();
	});
	triangle.addEventListener("reset", (e) => {
		e.preventDefault();
		new_triangle();
	});
	document.querySelectorAll("button.reset").forEach((button) => {
		button.addEventListener("click", () => {
			const input = button.parentNode.parentNode.children[0].children[0];
			input.value = 0;
		});
	});
});
