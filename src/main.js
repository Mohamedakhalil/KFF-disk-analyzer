const { invoke } = window.__TAURI__.tauri;

function init_analysis() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  var path_in = document.querySelector("#path-input").value;
  draw_pie(path_in);
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#path-button")
    .addEventListener("click", () => init_analysis());
});

// const contextMenu = document.getElementById("context-menu");
// const scope = document.querySelector("#tree_view");
// scope.addEventListener("contextmenu", (event)=>{
// 	event.preventDefault();
// 	const { clientX: mouseX, clientY: mouseY} = event;
// 	contextMenu.style.top = `${mouseY}px`;
// 	contextMenu.style.left = `${mouseX}px`;
// 	contextMenu.classList.add("visible");	
// })

// scope.addEventListener("click",(e)=>{
// 	if(e.target.offsetParent != contextMenu){
// 		contextMenu.classList.remove("visible");
// 	}
// });



document.querySelector("#chart_container").addEventListener("contextmenu", function(ev) {
    ev.preventDefault();
	const arr = document.querySelector("#path-input").value.split("/");
	let size = arr.length;
	let path="";
	for(let i = 1; i < size -1;i++){
		path = path+"/"+arr[i];
	}
	draw_pie(path);
	document.querySelector("#path-input").value = path;   
},false);
  
document.body.addEventListener("click", function(e) {
	const contextMenu = document.getElementById("context-menu");
	if(contextMenu.classList.contains('visible'))
	{
		contextMenu.classList.remove('visible');
	}
});  

var row_name;
//drawing the pie chart
async function draw_pie(path) {
	var arr = await invoke("get_paths", {dirPath: path});
	var points_json = [];
	for (var i = 0; i < arr.length; i++) {
	points_json.push({y: parseInt(arr[i]["size"]), indexLabel: arr[i]["path"]})
	}
	var chart = new CanvasJS.Chart("chart_container",
	{
		title:{
			text: "Disk Analysis"
		},
		legend: {
			maxWidth: 350,
			itemWidth: 120
		},
		data: [
		{
			type: "doughnut",
			showInLegend: true,
			legendText: "{indexLabel}",
      		click : onClick,
			dataPoints: points_json
		}
		]
	});
	chart.render();
  	async function onClick(e){
		draw_pie(path + "/" + e.dataPoint.indexLabel);
		document.querySelector("#path-input").value = path + "/" + e.dataPoint.indexLabel;   
	}
	var table_data = [];
	for (var i = 0; i < arr.length; i++) {
		table_data.push({name: arr[i]["path"], size:arr[i]["size"], age: arr[i]["age"], len:arr[i]["len"]});
	}
	var table = new Tabulator("#tree_view", {
		data:table_data,
		autoColumns:true,
		selectable: 1,
		autoColumnsDefinitions:function(definitions){
			//definitions - array of column definition objects
			definitions.forEach((column) => {
				column.headerFilter = true; // add header filter to every column
				
			});
	
			return definitions;
		},
	});
	table.on("rowClick", function(e, row){
		draw_pie(path+'/'+row.getData().name);
		document.querySelector("#path-input").value = path + "/" + row.getData().name;
	});
	const contextMenu = document.getElementById("context-menu");
	table.on("rowContext", function(e, row){
		e.preventDefault();
		row_name = row.getData().name;
		const { clientX: mouseX, clientY: mouseY} = e;
		contextMenu.style.top = `${mouseY}px`;
		contextMenu.style.left = `${mouseX}px`;
		contextMenu.classList.add("visible");	
	});
	
}

const contextMenu = document.getElementById("context-menu");
contextMenu.querySelector("#Del").addEventListener("click", function(){
	let path = document.querySelector("#path-input").value
	invoke("delete", {dirFilPath: path + "/" + row_name});
	draw_pie(path);
});

contextMenu.querySelector("#Back").addEventListener("click", function(){
	//console.log(path + "/" + row_name);
	const arr = document.querySelector("#path-input").value.split("/");
	let size = arr.length;
	let path="";
	for(let i = 1; i < size -1;i++){
		path = path+"/"+arr[i];
	}
	draw_pie(path);
	document.querySelector("#path-input").value = path;
});


/*
async function onClick(e){
	draw_pie(path + "/" + e.dataPoint.indexLabel);
	document.querySelector("#path-input").value = path + "/" + e.dataPoint.indexLabel;   
}
*/

var download = document.querySelector("#download-chart");

download.addEventListener('click', function (e) {
	var canvas = document.querySelector(".canvasjs-chart-canvas");
	var href = canvas.toDataURL("image/png");
	invoke("to_image", {base:href});
    // document.write('<img src="'+img+'"/>');
  });

  function delete_menu(){

  }