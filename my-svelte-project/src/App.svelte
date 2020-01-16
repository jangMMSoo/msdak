<script>
	export let name;

	let matrix = [
			 [1,2,3], [1,2,3], [1,2,3], //0, 1, 2
			 [4,5,6], [4,5,6], [1,2,3], //3, 4, 5
			 [7,8,9], [7,8,9], [1,2,3], //6, 7, 8

			 [1,2,3], [1,2,3], [1,2,3], // 9,10,11
			 [1,2,3], [1,2,3], [1,2,3], //12,13,14
			 [1,2,3], [1,2,3], [1,2,3], //15,16,17

			 [1,2,3], [1,2,3], [1,2,3], //18,19,20
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
</style>


<div class="content">
	<div class="row">
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[0][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[0][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[0][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[1][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[1][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[1][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[2][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[2][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[2][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[3][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[3][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[3][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[4][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[4][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[4][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[5][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[5][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[5][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[6][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[6][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[6][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[7][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[7][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[7][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[8][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[8][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[8][2]} min=1 max=9/></p>
		</div>
	</div>
	<div class="row">
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[9][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[9][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[9][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[10][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[10][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[10][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[11][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[11][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[11][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[12][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[12][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[12][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[13][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[13][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[13][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[14][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[14][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[14][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[15][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[15][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[15][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[16][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[16][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[16][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[17][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[17][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[17][2]} min=1 max=9/></p>
		</div>
	</div>
	<div class="row">
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[18][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[18][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[18][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[19][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[19][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[19][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[20][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[20][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[20][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[21][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[21][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[21][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[22][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[22][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[22][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[23][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[23][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[23][2]} min=1 max=9/></p>
		</div>
		<div class="large-squre">
			<p><input class='squre' type=number value={matrix[24][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[24][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[24][2]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[25][0]}  min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[25][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[25][2]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[26][0]} min=1 max=9/></p>
			<p><input class='squre' type=number value={matrix[26][1]} min=1 max=9/></p>
			<p><input class='squre-last' type=number value={matrix[26][2]} min=1 max=9/></p>
		</div>
	</div>
</div>