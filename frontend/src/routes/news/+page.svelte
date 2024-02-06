<svelte:head>
	<title>Neuigkeiten - ReviveIT</title>
</svelte:head>

<script>
	import { onDestroy, onMount } from "svelte";
	import config from "../../config.json"; 
	
	/**
	 * weird workaround cause ts cannot be used here
	 * @type {import("../../models/Article").Article[]}
	 */
	let articles;
	let showWarning = false;

	onMount(async () => {
		await fetch(`${config.apiUrl}/articles`).then(response => response.json()).then(response => {
			articles = response;
		}, _ => showWarning = true);
	});
</script>

<div class="content">
	<h1>Neuigkeiten</h1>

	{#if showWarning}
		<h1>Die Neuigkeiten konnten nicht geladen werden. Bitte laden sie die Seite neu.</h1>
	{:else}
		{#if !articles || articles.length === 0}
			<h1>Die Neuigkeiten sind am laden...</h1>
		{:else}
			{#each articles as article}
				<article>
					<div style="display: flex; align-items: center;">
						<h2 style="margin-right: 1rem;">{article.title}</h2>
						{article.date}
					</div>
					<p>{article.content}</p>
				</article>
			{/each}
		{/if}
	{/if}

	<!--article>
		<div style="display: flex; align-items: center;">
			<h2 style="margin-right: 1rem;">Website ist online!</h2>
			23.11.2023
		</div>
		<p>Unsere selbstgemachte Website kann nun im World Wide Web betrachtet werden.</p>
	</article-->
</div>

<style>
	article:not(:last-of-type) {
		border-bottom: 1px solid white;
	}
</style>
