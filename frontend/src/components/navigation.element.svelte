<script>
	import { page } from '$app/stores';

	export let path = '/';
	export let name = 'm152';
</script>

<a href={path} class={$page.url.pathname === path ? 'active' : ''}>
	{#if path !== '/'}
		{name}
	{:else}
		<img src="reviveit.logo.png" alt="ReviveIT logo" height="96px" />
	{/if}
</a>

<style>
	a:hover {
		background-color: #800080;
	}

	a:link {
		text-decoration: inherit;
		color: inherit;
	}

	a:visited {
		text-decoration: inherit;
		color: inherit;
	}

	a {
		width: 100%;
		font-size: 1.5em;
		font-weight: bold;
		padding: 0.7em 0;
	}

	.active {
		background-color: #800080;
		cursor: pointer;
	}

	img {
		margin-top: 16px;
		margin-bottom: 16px;
		filter: drop-shadow(3px 3px 0 black);
	}
</style>
