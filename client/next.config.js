module.exports = {
    reactStrictMode: true,
    async redirects() {
        return [
            {
                source: '/',
                destination: '/feed/all',
                permanent: true,
            },
        ]
    },
};
