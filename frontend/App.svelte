<script charset="utf-8">
    // URL del API de los gatitos
    const api_url = 'http://127.0.0.1:8000/api/cats';
    // Get api info
    const fetchTestimonials = (async () => {
        const response = await fetch(api_url)
        return await response.json()
    })()
</script>

<style type="text/css" media="screen">
    
</style>

<main>

</main>
