<script>
	export let name;

	let count = 1;
	// the `$:` means 're-run whenever these values change'
	$: doubled = count * 2;
	$: quadrupled = doubled * 2;
	let html = '';

	function handleClick() {
		count += 1;
	}


	function handleKeyDown(event) {
		if (event.keyCode === 13) {
			// Cancel the default action, if needed
			event.preventDefault();
			// Trigger the button element with a click
			count += 1;
			this.value = '';
			addLi();
		}
	}

	function addLi() {
		let answer = count * 2;
		html = '<li><span style="color: red;">' + count + '.</span> '  + count +' x ' + '2 = ' + answer +'</li>' + html;
	}
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
		<button >Answer Count: {count}</button>
		<button >Wrong Count: {count}</button>
		</div>
	</div>

	<div class="center">
		<p class="font-left">{count} x 2 </p>
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


