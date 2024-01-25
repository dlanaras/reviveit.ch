<script>
	import { page } from '$app/stores';

	export let path = '/';
	export let name = '';
</script>

{#if path !== '/'}
	<a href={path} class={$page.url.pathname === path ? 'active' : ''}>
		{name}
	</a>
{:else}
	<a href={path} class={$page.url.pathname === path ? 'logo active' : 'logo'}>
		<img src="reviveit.logo.png" alt="ReviveIT logo" height="96px" />
	</a>
{/if}

<style>
	a:hover {
		text-decoration: underline !important;
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
		font-weight: normal;
		cursor: pointer;
		text-decoration: underline;
		padding: 0.5em 0;
	}

	a.logo {
		margin: 0;
		padding: 0 !important;
	}

	.active {
		font-weight: bold;
	}

	img {
		margin: 8px;
		filter: drop-shadow(2px 2px 0 black);
	}
</style>
