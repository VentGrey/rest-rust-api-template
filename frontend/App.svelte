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
        display: grid;
        grid-template-columns: auto auto auto auto;
        grid-template-rows: auto auto auto auto;
        justify-content: space-evenly;
        align-content: center;
    }

    .card {
        margin: 1.3em;
        padding: 1.3em;
        border-radius: 1.5em;
        box-shadow: 1px 1px 2px 1px rgba(0, 0, 0, 0.3);
    }

    .img-gato {
        display: inline-block;
        max-width: 100%;
        max-height: 100%;
        height: 45%;
        width: 45%;
        border-radius: 1.5em;
    }

    .cat-name{
        display: block;
        color: #452981;
        text-shadow: 1px 1px 1px #452981;
    }

    .adopted {
        display: inline-block;
        padding: 5px;
        background-color: #3a9104;
        background: linear-gradient(90deg, rgba(88,230,0,1) 0%, rgba(58,145,4,1) 100%);
        box-shadow: 1px 1px 2px 1px rgba(58, 145, 4, 0.3);
        color: #FAFAFA;
        border-radius: 0.5em;
        font-size: 0.9em;
    }

    .noadopted {
        display: inline-block;
        padding: 5px;
        background-color: #002e99;
        background: linear-gradient(90deg, rgba(0,77,255,1) 0%, rgba(0,46,153,1) 100%);
        box-shadow: 1px 1px 2px 1px rgba(0, 46, 153, 0.3);
        color: #FAFAFA;
        border-radius: 0.5em;
        font-size: 0.9em;
    }

    .cat-desc {
        color: #666666;
        text-overflow: ellipsis;
        word-break: break-all;
        overflow-wrap: break-word;
    }
</style>

<main>
    <h1 align="center">Base de Gatos 😺</h1>

    <div class="container">

        {#await fetchCats}
            <p>Cargando gatos...</p>
        {:then data}
        <div class="card">
            <img class="img-gato" src="https://www.rover.com/blog/wp-content/uploads/2019/06/sitting-siamese-cat-960x540.jpg" alt="Foto del gatito"/>
            <h2 class="cat-name">Nombre del gatito</h2>
            <h3 class="adopted">Adoptado ❤️</h3>
            <h3 class="noadopted">Buscando un hogar 🏠</h3>
            <hr>
            <p class="cat-desc">Descripción del gato en cuestión. Las descripciones de los gatos largas son más convenientes para que las personas decidan adoptar a los gatitos que les presentemos.</p>
        </div>

        {:catch error}
            <p>Ocurrió un error al cargar los gatos :(</p>
        {/await}
    </div>
</main>
