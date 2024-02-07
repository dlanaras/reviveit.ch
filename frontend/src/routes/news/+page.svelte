<svelte:head>
	<title>Neuigkeiten - ReviveIT</title>
</svelte:head>

<script>
	/** @type {import('./$types').PageServerData}*/
	export let data;
</script>

<div class="content">
	<h1>Neuigkeiten</h1>
	{#if !data?.articles || data.articles.length === 0}
		<h1>Die Neuigkeiten sind am laden...</h1>
	{:else}
		{#each data.articles as article}
			<article>
				<div style="display: flex; align-items: center;">
					<h2 style="margin-right: 1rem;">{article.title}</h2>
					{new Date(article.date*1000).toLocaleDateString('de-CH')}
				</div>
				<p>{article.content}</p>
			</article>
		{/each}
	{/if}
</div>

<style>
	article:not(:last-of-type) {
		border-bottom: 1px solid white;
	}
</style>
