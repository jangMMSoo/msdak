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
		html = html + '<li>' + count +' * ' + '2 = ' + answer +'</li>';
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

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>


<button on:click={handleClick}>
	Count: {count}
</button>

<p>{count} * 2 </p>
<p>= </p><input on:keydown={handleKeyDown} />

<ul contenteditable="true"  bind:innerHTML={html}></ul>

