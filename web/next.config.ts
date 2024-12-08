import type { NextConfig } from 'next'

const checkEnvironments = () => {
  const requiredEnvs = [
    'NEXT_PUBLIC_CASDOOR_SERVER_URL',
    'NEXT_PUBLIC_CASDOOR_CLIENT_ID',
    'NEXT_PUBLIC_CASDOOR_APP_NAME',
    'NEXT_PUBLIC_CASDOOR_ORG_NAME',
    'JWT_KEY_PATH',
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