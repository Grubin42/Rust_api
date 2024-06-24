document.addEventListener('DOMContentLoaded', async () => {
    $path = window.location;

    // TODO: split ton url

    // TODO: check current slug
    $slug

    const response = await fetch('http://localhost:8000/api/register', {
        method: 'GET',
        headers: {
            'Authorization': 'Bearer token', // Replace with your token logic
        },
    });

    if (!response.ok) {
        window.location.href = '/html/login.html';
    }
});

document.getElementById('logout-button').addEventListener('click', () => {
    // Logic to clear the token or session
    window.location.href = '/html/login.html';
});