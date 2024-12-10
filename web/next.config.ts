import type { NextConfig } from 'next'

const checkEnvironments = () => {
  const requiredEnvs = [
    "NEXT_PUBLIC_LIBRE_BACKEND_URL",
  ];
  requiredEnvs.forEach((env) => {
    if (!process.env[env]) {
      throw new Error(`Environment variable ${env} is not set`);
    }
  });
};

checkEnvironments();

const nextConfig: NextConfig = {
  reactStrictMode: true,
  eslint: {
    ignoreDuringBuilds: true,
  },
  images: { unoptimized: true },
  output: "standalone",
};


module.exports = nextConfig;