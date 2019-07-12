run = async() => {
    const { render } = await import('../pkg');
    new render();
};

run();