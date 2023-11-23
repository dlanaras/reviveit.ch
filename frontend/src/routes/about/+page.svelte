<svelte:head>
	<title>Über uns - ReviveIT</title>
</svelte:head>

<div class="content">
	<h1>Über uns</h1>
	<p>
		Wir sind ReviveIT - eine einfache Gesellschaft, welche alte oder kaputte Computer kauft,
		repariert und billig an Personen verkauft, die diese brauchen können.
	</p>

	<h2>Wer sind wir?</h2>
	<div class="img-container">
		<figure>
			<img src="/img/sven.png" alt="Sven Oppliger" />
			<figcaption>Sven Oppliger - CEO</figcaption>
		</figure>
		<figure>
			<img src="/img/dimi.png" alt="Dimitrios Lanaras" />
			<figcaption>Dimitrios Lanaras - CPO Verwantwortlich für Einkauf und Reparaturen</figcaption>
		</figure>
		<figure>
			<img src="/img/nils.png" alt="Nils Hindermann" />
			<figcaption>Nils Hindermann - CFO Verantwortlich für Marketing, Verkauf und Finanzen</figcaption>
		</figure>
		<figure>
			<img src="/img/luna.png" alt="Luna Neff" />
			<figcaption>Luna Neff - CIO Verantwortlich für IT und Administration</figcaption>
		</figure>
	</div>
</div>

<style>
	.img-container {
		display: flex;
		justify-content: space-evenly;
		flex-wrap: wrap;
	}

	img {
		border-radius: 10px;
	}

	figcaption, img {
		width: 250px;
	}
</style>
