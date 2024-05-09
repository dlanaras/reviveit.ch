<svelte:head>
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
</svelte:head>

<div id="footer">
	<p><a href="https://reviveit.ch" class="link">reviveit.ch</a></p>
	<p>
		ReviveIT Team<br />
		<a href="/imprint">Impressum</a>&nbsp;&middot;&nbsp;<a href="/privacy-policy"
			>Datenschutzerklärung</a
		>&nbsp;&middot;&nbsp;<a href="https://www.linkedin.com/company/reviveit-ch/about/" target="_blank" class="fa fa-linkedin"/>&nbsp;&middot;&nbsp;
		<a href="https://www.youtube.com/channel/UCVoNLK0OEj2DMcECFFJ4yUA/about" target="_blank" class="fa fa-youtube" />
	</p>
	<p>
		<span style="rotate: 180deg; display: inline-block">&copy;</span> GPLv3 {new Date().getFullYear()}
	</p>
</div>

<style>
	div {
		background-color: #614880;
		display: flex;
		width: 100%;
		overflow: hidden;
		justify-content: space-around;
		align-items: center;
		flex-direction: column;
		flex-shrink: 0;
		flex-wrap: wrap;
	}

	p {
		text-align: center;
	}

	a {
		text-decoration: underline;
	}

	a.link {
		text-decoration: none !important;
	}

	.fa {
		text-decoration: none;
	}

	@media screen and (min-width: 600px) {
		div {
			flex-direction: row;
		}
	}
</style>
