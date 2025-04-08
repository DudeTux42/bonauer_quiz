// Create the public directory if it doesn't exist
// mkdir -p public

// This script loads after the WASM is ready
window.addEventListener('load', async function() {
    try {
        document.getElementById('loading').innerHTML = 'Finding WASM module...';
        
        // Find the generated JS file
        const scriptTags = document.querySelectorAll('script');
        let wasmModule = null;
        
        for (const script of scriptTags) {
            if (script.src && script.src.includes('bonauer_quiz-')) {
                const modulePath = script.src;
                document.getElementById('loading').innerHTML = 'Loading WASM module...';
                console.log('Found WASM module at:', modulePath);
                
                // Dynamically import the module
                const module = await import(modulePath);
                wasmModule = module;
                break;
            }
        }
        
        if (!wasmModule) {
            throw new Error('Could not find WASM module script');
        }
        
        document.getElementById('loading').innerHTML = 'Initializing application...';
        
        // Initialize the WASM module
        await wasmModule.default();
        
        // Start the application if start function exists
        if (wasmModule && typeof wasmModule.start === 'function') {
            wasmModule.start('canvas');
            document.getElementById('loading').style.display = 'none';
        } else {
            throw new Error('start function not found in WASM module');
        }
    } catch (error) {
        console.error('Failed to initialize application:', error);
        document.getElementById('loading').innerHTML = 'Error: ' + error.message;
    }
});
