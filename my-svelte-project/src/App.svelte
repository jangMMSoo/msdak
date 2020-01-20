<script>
	export let name;

	let answerMatrix = [
			 [1,2,3], [1,2,3], [1,2,3], //0, 1, 2
			 [4,5,6], [4,5,6], [1,2,3], //3, 4, 5
			 [7,8,9], [7,8,9], [1,2,3], //6, 7, 8

			 [1,2,3], [1,2,3], [1,2,3], // 9,10,11
			 [1,2,3], [1,2,3], [1,2,3], //12,13,14
			 [1,2,3], [1,2,3], [1,2,3], //15,16,17

			 [1,2,3], [1,2,3], [1,2,3], //18,19,20
			 [1,2,3], [1,2,3], [1,2,3], //21,22,23
			 [1,2,3], [1,2,3], [3,2,3]];//24,25,26

	let questionMatrix = [
			[1,2,3], [1,2,3], [1,2,3], //0, 1, 2
			[4,'',6], [4,5,6], [1,2,3], //3, 4, 5
			[7,8,9], [7,8,9], [1,2,3], //6, 7, 8

			[1,2,3], [1,2,''], [1,2,3], // 9,10,11
			[1,2,3], [1,2,3], [1,2,3], //12,13,14
			[1,2,3], [1,2,''], [1,2,3], //15,16,17

			[1,'',3], [1,2,''], [1,2,3], //18,19,20
			[1,2,3], [1,2,3], [1,2,3], //21,22,23
			[1,2,3], [1,2,3], [3,2,3]];//24,25,26


</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}

	.squre {
		float : left;
		width : 50px;
		margin-right:2px;
		margin-left:2px;
	}

	.squre-last {
		width : 50px;
		margin-left:2px;
	}

	.large-squre {
		width: 170px;
		float: left;
		border: #0b2e13 3px solid;
		margin: 1px 1px 1px 1px;
	}

	.row {
		display: flex;
	}

	.content {
		margin: 0 auto;
		width: 520px;
	}

	/* Hide HTML5 Up and Down arrows. */
	input[type="number"]::-webkit-outer-spin-button, input[type="number"]::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	input[type="number"] {
		-moz-appearance: textfield;
		text-align: center;
	}
</style>


<div class="content">
	<div class="row">
		<div class="large-squre">
			<p><input id='0-0' class='squre' type=number value={questionMatrix[0][0]} min=1 max=9/></p>
			<p><input id='0-1' class='squre' type=number value={questionMatrix[0][1]} min=1 max=9/></p>
			<p><input id='0-2'  class='squre-last' type=number value={questionMatrix[0][2]}  min=1 max=9/></p>
			<p><input id='1-0' class='squre' type=number value={questionMatrix[1][0]}  min=1 max=9/></p>
			<p><input id='1-1' class='squre' type=number value={questionMatrix[1][1]} min=1 max=9/></p>
			<p><input id='1-2'  class='squre-last' type=number value={questionMatrix[1][2]} min=1 max=9/></p>
			<p><input id='2-0' class='squre' type=number value={questionMatrix[2][0]} min=1 max=9/></p>
			<p><input id='2-1' class='squre' type=number value={questionMatrix[2][1]} min=1 max=9/></p>
			<p><input id='2-2'  class='squre-last' type=number value={questionMatrix[2][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input id='3-0' class='squre' type=number value={questionMatrix[3][0]} min=1 max=9/></p>
			<p><input id='3-1' class='squre' type=number value={questionMatrix[3][1]} min=1 max=9/></p>
			<p><input id='3-2'  class='squre-last' type=number value={questionMatrix[3][2]}  min=1 max=9/></p>
			<p><input id='4-0' class='squre' type=number value={questionMatrix[4][0]}  min=1 max=9/></p>
			<p><input id='4-1' class='squre' type=number value={questionMatrix[4][1]} min=1 max=9/></p>
			<p><input id='4-2'  class='squre-last' type=number value={questionMatrix[4][2]} min=1 max=9/></p>
			<p><input id='5-0' class='squre' type=number value={questionMatrix[5][0]} min=1 max=9/></p>
			<p><input id='5-1' class='squre' type=number value={questionMatrix[5][1]} min=1 max=9/></p>
			<p><input id='5-2'  class='squre-last' type=number value={questionMatrix[5][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input id='6-0' class='squre' type=number value={questionMatrix[6][0]} min=1 max=9/></p>
			<p><input id='6-1' class='squre' type=number value={questionMatrix[6][1]} min=1 max=9/></p>
			<p><input id='6-2'  class='squre-last' type=number value={questionMatrix[6][2]}  min=1 max=9/></p>
			<p><input id='7-0' class='squre' type=number value={questionMatrix[7][0]}  min=1 max=9/></p>
			<p><input id='7-1' class='squre' type=number value={questionMatrix[7][1]} min=1 max=9/></p>
			<p><input id='7-2'  class='squre-last' type=number value={questionMatrix[7][2]} min=1 max=9/></p>
			<p><input id='8-0' class='squre' type=number value={questionMatrix[8][0]} min=1 max=9/></p>
			<p><input id='8-1' class='squre' type=number value={questionMatrix[8][1]} min=1 max=9/></p>
			<p><input id='8-2'  class='squre-last' type=number value={questionMatrix[8][2]} min=1 max=9/></p>
		</div>
	</div>
	<div class="row">
		<div class="large-squre">
			<p><input id='9-0' class='squre' type=number value={questionMatrix[9][0]} min=1 max=9/></p>
			<p><input id='9-1' class='squre' type=number value={questionMatrix[9][1]} min=1 max=9/></p>
			<p><input id='9-2'  class='squre-last' type=number value={questionMatrix[9][2]}  min=1 max=9/></p>
			<p><input id='10-0' class='squre' type=number value={questionMatrix[10][0]}  min=1 max=9/></p>
			<p><input id='10-1' class='squre' type=number value={questionMatrix[10][1]} min=1 max=9/></p>
			<p><input id='10-2'  class='squre-last' type=number value={questionMatrix[10][2]} min=1 max=9/></p>
			<p><input id='11-0' class='squre' type=number value={questionMatrix[11][0]} min=1 max=9/></p>
			<p><input id='11-1' class='squre' type=number value={questionMatrix[11][1]} min=1 max=9/></p>
			<p><input id='11-2'  class='squre-last' type=number value={questionMatrix[11][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input id='12-0' class='squre' type=number value={questionMatrix[12][0]} min=1 max=9/></p>
			<p><input id='12-1' class='squre' type=number value={questionMatrix[12][1]} min=1 max=9/></p>
			<p><input  id='12-2'  class='squre-last' type=number value={questionMatrix[12][2]}  min=1 max=9/></p>
			<p><input id='13-0' class='squre' type=number value={questionMatrix[13][0]}  min=1 max=9/></p>
			<p><input id='13-1' class='squre' type=number value={questionMatrix[13][1]} min=1 max=9/></p>
			<p><input  id='13-2'  class='squre-last' type=number value={questionMatrix[13][2]} min=1 max=9/></p>
			<p><input id='14-0' class='squre' type=number value={questionMatrix[14][0]} min=1 max=9/></p>
			<p><input id='14-1' class='squre' type=number value={questionMatrix[14][1]} min=1 max=9/></p>
			<p><input  id='14-2'  class='squre-last' type=number value={questionMatrix[14][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input id='15-0' class='squre' type=number value={questionMatrix[15][0]} min=1 max=9/></p>
			<p><input id='15-1' class='squre' type=number value={questionMatrix[15][1]} min=1 max=9/></p>
			<p><input id='15-2'  class='squre-last' type=number value={questionMatrix[15][2]}  min=1 max=9/></p>
			<p><input id='16-0' class='squre' type=number value={questionMatrix[16][0]}  min=1 max=9/></p>
			<p><input id='16-1' class='squre' type=number value={questionMatrix[16][1]} min=1 max=9/></p>
			<p><input id='16-2'  class='squre-last' type=number value={questionMatrix[16][2]} min=1 max=9/></p>
			<p><input id='17-0' class='squre' type=number value={questionMatrix[17][0]} min=1 max=9/></p>
			<p><input id='17-1' class='squre' type=number value={questionMatrix[17][1]} min=1 max=9/></p>
			<p><input id='17-2'  class='squre-last' type=number value={questionMatrix[17][2]} min=1 max=9/></p>
		</div>
	</div>
	<div class="row">
		<div class="large-squre">
			<p><input id='18-0' class='squre' type=number value={questionMatrix[18][0]} min=1 max=9/></p>
			<p><input id='18-1' class='squre' type=number value={questionMatrix[18][1]} min=1 max=9/></p>
			<p><input id='18-2'  class='squre-last' type=number value={questionMatrix[18][2]}  min=1 max=9/></p>
			<p><input id='19-0' class='squre' type=number value={questionMatrix[19][0]}  min=1 max=9/></p>
			<p><input id='19-1' class='squre' type=number value={questionMatrix[19][1]} min=1 max=9/></p>
			<p><input id='19-2'  class='squre-last' type=number value={questionMatrix[19][2]} min=1 max=9/></p>
			<p><input id='20-0' class='squre' type=number value={questionMatrix[20][0]} min=1 max=9/></p>
			<p><input id='20-1' class='squre' type=number value={questionMatrix[20][1]} min=1 max=9/></p>
			<p><input id='20-2'  class='squre-last' type=number value={questionMatrix[20][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input id='21-0' class='squre' type=number value={questionMatrix[21][0]} min=1 max=9/></p>
			<p><input id='21-1' class='squre' type=number value={questionMatrix[21][1]} min=1 max=9/></p>
			<p><input id='21-2'  class='squre-last' type=number value={questionMatrix[21][2]}  min=1 max=9/></p>
			<p><input id='22-0' class='squre' type=number value={questionMatrix[22][0]}  min=1 max=9/></p>
			<p><input id='22-1' class='squre' type=number value={questionMatrix[22][1]} min=1 max=9/></p>
			<p><input id='22-2'  class='squre-last' type=number value={questionMatrix[22][2]} min=1 max=9/></p>
			<p><input id='23-0' class='squre' type=number value={questionMatrix[23][0]} min=1 max=9/></p>
			<p><input id='23-1' class='squre' type=number value={questionMatrix[23][1]} min=1 max=9/></p>
			<p><input id='23-2'  class='squre-last' type=number value={questionMatrix[23][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input id='24-0' class='squre' type=number value={questionMatrix[24][0]} min=1 max=9/></p>
			<p><input id='24-1' class='squre' type=number value={questionMatrix[24][1]} min=1 max=9/></p>
			<p><input id='24-2'  class='squre-last' type=number value={questionMatrix[24][2]}  min=1 max=9/></p>
			<p><input id='25-0' class='squre' type=number value={questionMatrix[25][0]}  min=1 max=9/></p>
			<p><input id='25-1' class='squre' type=number value={questionMatrix[25][1]} min=1 max=9/></p>
			<p><input id='25-2'  class='squre-last' type=number value={questionMatrix[25][2]} min=1 max=9/></p>
			<p><input id='26-0' class='squre' type=number value={questionMatrix[26][0]} min=1 max=9/></p>
			<p><input id='26-1' class='squre' type=number value={questionMatrix[26][1]} min=1 max=9/></p>
			<p><input id='26-2'  class='squre-last' type=number value={questionMatrix[26][2]} min=1 max=9/></p>
		</div>
	</div>
</div>