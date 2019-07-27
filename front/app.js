run = async() => {
    const { render } = await import('../pkg')
    new render(process.env.API_URL)
};

run();