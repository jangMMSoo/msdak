<script>
	export let name;

	let source = 1;
	let target = 1;
	let count = 1;
	let wrongCount = 0;
	let correctCount = 0;
	let html = '';
	let min = 1;
	let max = 10;

	function initRand() {
		source = getRandomArbitrary(min, max);
		target = getRandomArbitrary(min, max);
	}

	function getRandomArbitrary(min, max) {
		return Math.floor(Math.random() * (max - min) + min);
	}


	function handleKeyDown(event) {
		if (event.keyCode === 13) {
			// Cancel the default action, if needed
			event.preventDefault();
			// Trigger the button element with a click
			addLi(this.value);
			this.value = '';
			initRand();
			count++;
		}
	}

	function addLi(answer) {
		let isAnswer = function(color, target, answer){
			let isAnswer = color * target === Number(answer);
			if(isAnswer){
				correctCount++;
			}else {
				wrongCount++;
			}
			return isAnswer;
		};
		let color = isAnswer(source, target, answer) ? 'blue' : 'red';
		html = '<li style="color:'+color+'"><span>' + count + '.</span> '  + source +' x ' +  target+ ' = ' + answer +'</li>' + html;
	}

	initRand();

</script>

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

	.center {
		margin: auto;
		width: 50%;
		/*border: 3px solid green;*/
		padding: 10px;
	}

	.left {
		margin: auto auto auto 0px;
		width: 65%;
		/*border: 3px solid green;*/
		padding: 10px;
		text-align: left;
	}

	.font-left {
		font-size: 128px;
		margin: 0;
		text-align: left;
	}


	.answer-left {
		font-size: 24px;
		margin: 0;
		text-align: left;
	}

	.right {
		margin: auto 0px auto auto;
		width: 35%;
		/*border: 3px solid green;*/
		padding: 10px;
	}

	.answer-list {
		list-style-type: none;
		text-align: left;
	}

	.div-line {
		border-top: 1px solid;
	}
	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>

	<div class="center">
		<div class="left">
		<button >Answer Count: {correctCount}</button>
		<button >Wrong Count: {wrongCount}</button>
		</div>
	</div>

	<div class="center">
		<p class="font-left">{source} x {target} </p>
		<p class="answer-left">= <input on:keydown={handleKeyDown} /></p>
	</div>

	<div class="div-line">
		<div class="center">
			<div class="right">
				<ul class="answer-list" contenteditable="true"  bind:innerHTML={html}></ul>
			</div>
		</div>
	</div>
</main>


