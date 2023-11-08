import { useConfig, useFaceless, initializeFaceless } from "@/store";
import { eras, medias } from "@/components/community";
import { platform_id } from "@/utils/account";
import pinia from "@/utils/piniaInstance";
import phonePlatform from "@/assets/images/phonePlatform.png";

const createAccount = async (platform: string, username: string, msk?: string) => {

  const faceless = useFaceless(pinia);
  await initializeFaceless(msk);

  if (platform && username) {
    let pid = platform_id(platform, username);
    eras[pid] = { icon: phonePlatform, platform, username };
    medias[pid] = {
      avatar: "https://avatars.githubusercontent.com/u/2106987?v=4",
      username: username,
      platform: platform,
      amount: await faceless.client.balance(platform, username),
      coin: "ETH",
      key: medias.length + 1,
    };
    await faceless.client.register(platform, username);
  }
}

const deriveKey = async (ids: string[]) => {
  const faceless = useFaceless(pinia);
  await initializeFaceless();

  const sks = faceless.client.derive(ids);
  return sks;
}

const recoverKey = async (ids: string[], sks: string[]) => {
  const faceless = useFaceless(pinia);
  await initializeFaceless();

  const msk = faceless.client.recover(ids, sks);
  return msk;
}

export { createAccount, deriveKey, recoverKey };
