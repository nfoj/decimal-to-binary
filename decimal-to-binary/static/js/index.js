document.getElementById('binaryForm').onsubmit = async (e) => {
    e.preventDefault();
    const number = document.getElementById('number').value;

    const response = await fetch('/convert', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ number: parseInt(number) }),
    });

    const result = await response.text();
    document.getElementById('result').innerText = `Resultado: ${result}`;
};