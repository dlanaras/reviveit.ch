<div id="footer">
	<p><a href="https://reviveit.ch" class="link">reviveit.ch</a></p>
	<p>
		ReviveIT Team<br />
		<a href="/imprint">Impressum</a>&nbsp;&middot;&nbsp;<a href="/privacy-policy"
			>Datenschutzerklärung</a
		>
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

	@media screen and (min-width: 600px) {
		div {
			flex-direction: row;
		}
	}
</style>
