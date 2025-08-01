import type { AppInitConfig } from '~/types/app.types';
import { defaultLocale, supportedLocales } from './i18n.config';
import { isSemanticVersion, removeBasePathFromPathname } from '~/utils/helper.utils';

const baseUrl = import.meta.env.BASE_URL || '/';

let versionedBaseUrl = baseUrl;
const parsedPath = removeBasePathFromPathname(location.pathname, baseUrl);
const parts = parsedPath.split('/').filter(Boolean);

if (parts.length && isSemanticVersion(parts[0], 'v')) {
  versionedBaseUrl = `${baseUrl}${parts[0]}/`;
}

const getHttpGatewayUrl =
  (isProduction: boolean) =>
  (canisterId: string): URL => {
    return isProduction
      ? new URL(`https://${canisterId}.icp0.io`)
      : new URL(`http://localhost:4943?canisterId=${canisterId}`);
  };

const appInitConfig: AppInitConfig = {
  name: import.meta.env.APP_TITLE || 'Orbit',
  version: import.meta.env.APP_VERSION || '0.0.0',
  logLevel: import.meta.env.APP_LOG_LEVEL || 'info',
  baseUrl: import.meta.env.BASE_URL || '/',
  versionedBaseUrl,
  buildMode: import.meta.env.APP_BUILD_MODE || 'production',
  isProduction: !!import.meta.env.PROD,
  apiGatewayUrl: new URL(import.meta.env.PROD ? 'https://icp-api.io' : 'http://localhost:4943'),
  httpGatewayUrl: getHttpGatewayUrl(import.meta.env.PROD),
  derivationOrigin: import.meta.env.PROD ? 'https://orbitwallet.io' : undefined,
  marketingSiteUrl: import.meta.env.APP_MARKETING_SITE_URL,
  locale: {
    default: defaultLocale,
    supportedLocales,
  },
  providers: {
    internetIdentity: import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY,
  },
  canisters: {
    app_wallet: import.meta.env.APP_CANISTER_ID_APP_WALLET,
    controlPanel: import.meta.env.APP_CANISTER_ID_CONTROL_PANEL,
    internetIdentity: import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY,
    icpIndex: import.meta.env.APP_CANISTER_ID_ICP_INDEX,
  },
};

export { appInitConfig };
