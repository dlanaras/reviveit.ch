<script>
	import Header from '../components/header.svelte';
	import Footer from '../components/footer.svelte';
</script>

<div id="wrapper">
	<Header />
	<slot class="test" />
	<Footer />
</div>

<style>
	#wrapper {
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow: auto;
	}
</style>
