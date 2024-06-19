const { getGeminiData } = require('./native/index.node'); 

async function test() {
    try {
        const data = await getGeminiData();
        console.log('API Data:', data);
    } catch (err) {
        console.error('Error:', err);
    }
}

test();
