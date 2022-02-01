<script charset="utf-8">
    // URL del API de los gatitos
    const api_url = 'http://127.0.0.1:8000/api/cats';
    // Get api info
    const fetchCats = (async () => {
        const response = await fetch(api_url)
        return await response.json()
    })()
</script>

<style type="text/css" media="screen">
    @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@100;300;400;700;900&display=swap');

    * {
        font-family: 'Poppins', sans-serif;
    }

    .container {
        align-content: center;
        display: grid;
        grid-template-columns: auto auto auto auto;
        grid-template-rows: auto auto auto auto;
        justify-content: space-evenly;
    }

    .card {
        border-radius: 1.5em;
        box-shadow: 1px 1px 2px 1px rgba(0, 0, 0, 0.3);
        margin: 1.3em;
        padding: 1.3em;
    }

    .img-gato {
        border-radius: 1.5em;
        display: inline-block;
        height: 45%;
        max-height: 100%;
        max-width: 100%;
        width: 45%;
    }

    .cat-name{
        color: #452981;
        display: block;
        text-shadow: 1px 1px 1px #452981;
    }

    .adopted {
        background-color: #3a9104;
        background: linear-gradient(90deg, rgba(88,230,0,1) 0%, rgba(58,145,4,1) 100%);
        border-radius: 0.5em;
        box-shadow: 1px 1px 2px 1px rgba(58, 145, 4, 0.3);
        color: #FAFAFA;
        display: inline-block;
        font-size: 0.9em;
        padding: 5px;
    }

    .noadopted {
        background-color: #002e99;
        background: linear-gradient(90deg, rgba(0,77,255,1) 0%, rgba(0,46,153,1) 100%);
        border-radius: 0.5em;
        box-shadow: 1px 1px 2px 1px rgba(0, 46, 153, 0.3);
        color: #FAFAFA;
        display: inline-block;
        font-size: 0.9em;
        padding: 5px;
    }

    .cat-desc {
        color: #666666;
        margin-bottom: 1.5em;
        margin-top: 1.5em;
        overflow-wrap: break-word;
        padding-bottom: 5em;
        text-overflow: ellipsis;
        word-break: break-all;
    }
</style>

<main>
    <h1 align="center">Base de Gatos 😺</h1>

    <div class="container">

        {#await fetchCats}
            <p>Cargando gatos...</p>
        {:then data}
            {#each data.result as cat}
                <div class="card">
                    <img class="img-gato" src={cat.photo_url} alt="Foto del gatito"/>
                    <h2 class="cat-name">{cat.name}</h2>
                    {#if cat.is_adopted}
                        <h3 class="adopted">Adoptado ❤️</h3>
                    {:else}
                        <h3 class="noadopted">Buscando un hogar 🏠</h3>
                    {/if}
                    <hr>
                    <p class="cat-desc">{cat.description}</p>
                </div>
            {/each}
        {:catch error}
            <p>Ocurrió un error al cargar los gatos :(</p>
        {/await}
    </div>
</main>
