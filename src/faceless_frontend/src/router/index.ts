import { useWallet } from "@/store";
import { createRouter, RouteRecordRaw, createWebHistory } from "vue-router";
import { createAccount, deriveKey, recoverKey } from "@/utils/registerAccount";

// OmniAuth
const callbackOmniAuth = async (to, from, next) => {
  const authHeaders = {
    headers: {
      "access-token": to.query.auth_token,
      client: to.query.client_id,
      uid: to.query.uid,
    },
    data: {
      data: {
        nickname: to.query.nickname,
      },
    },
  };

  let msk = localStorage.getItem("msk");

  if (to.params.provider === "google_oauth2") {
    await createAccount("Google", to.query.nickname, msk);
  }

  if (to.params.provider === "twitter") {
    await createAccount("Twitter", to.query.nickname, msk);
  }

  if (to.params.provider === "telegram") {
    await createAccount("Telegram", to.query.nickname, msk);
  }

  if (to.params.provider === "cancel") {
    await createAccount("", "");
  }

  next("/wallet");
};

// Register OmniAuth
const registerCallbackOmniAuth = async (to, from, next) => {
  const authHeaders = {
    headers: {
      "access-token": to.query.auth_token,
      client: to.query.client_id,
      uid: to.query.uid,
    },
    data: {
      data: {
        nickname: to.query.nickname,
      },
    },
  };

  if (to.params.provider === "google_oauth2") {
    localStorage.setItem("Google-reg", to.query.nickname);
    console.log("Google auth is validated. Nickname ", localStorage.getItem("Google-reg"));
  }

  if (to.params.provider === "twitter") {
    localStorage.setItem("Twitter-reg", to.query.nickname);
    console.log("Twitter auth is validated. Nickname: ", localStorage.getItem("Twitter-reg"));
  }

  if (to.params.provider === "telegram") {
    localStorage.setItem("Telegram-reg", to.query.nickname);
    console.log("Telegram auth is validated. Nickname: ", localStorage.getItem("Telegram-reg"));
  }

  if (to.params.provider === "cancel") {
  }

  if (localStorage.getItem("Google-reg") && localStorage.getItem("Twitter-reg") && localStorage.getItem("Telegram-reg")) {
    console.log("Three platforms are all validated. Start generating key...");
    const sks = await deriveKey(["Google", "Twitter", "Telegram"]);
    localStorage.setItem("msk", sks[0]);
    localStorage.setItem("Google_sk", sks[1]);
    localStorage.setItem("Twitter_sk", sks[2]);
    localStorage.setItem("Telegram_sk", sks[3]);
    console.log("Register successfully: ", sks[0]);
  }

  next("/wallet");
};

// Login OmniAuth
const loginCallbackOmniAuth = async (to, from, next) => {
  const authHeaders = {
    headers: {
      "access-token": to.query.auth_token,
      client: to.query.client_id,
      uid: to.query.uid,
    },
    data: {
      data: {
        nickname: to.query.nickname,
      },
    },
  };

  if (to.params.provider === "google_oauth2") {
    localStorage.setItem("Google", to.query.auth_token);
    console.log("Google auth is validated. Auth token: ", localStorage.getItem("Google"));
  }

  if (to.params.provider === "twitter") {
    localStorage.setItem("Twitter", to.query.auth_token);
    console.log("Twitter auth is validated.  Auth token: ", localStorage.getItem("Twitter"));
  }

  if (to.params.provider === "telegram") {
    localStorage.setItem("Telegram", to.query.auth_token);
    console.log("Telegram auth is validated.  Auth token: ", localStorage.getItem("Telegram"));
  }

  if (to.params.provider === "cancel") {
  }

  const validated_count = 
    Number(localStorage.getItem("Google") !== null) + 
    Number(localStorage.getItem("Twitter") !== null) +
    Number(localStorage.getItem("Telegram") !== null);

  if (validated_count >= 2) {
    console.log("Two platforms are validated. Start recovering key...");
    var platforms = ["Google", "Twitter", "Telegram"];
    var ids: string[] = [];
    var sks: string[] = [];
    for(var i in platforms) {
      if(localStorage.getItem(platforms[i]) !== null) {
        ids.push(platforms[i]);
        const sk = localStorage.getItem(platforms[i] + "_sk");
        if (sk)
          sks.push(sk);
        else 
          throw new Error("Cannot find sk: " + platforms[i]);
      }
    }
    const msk = await recoverKey(ids, sks);
    localStorage.setItem("msk", msk);
    console.log("Login successfully: ", msk);

    await createAccount("Google", localStorage.getItem("Google-reg"), sks[0]);
    await createAccount("Twitter", localStorage.getItem("Twitter-reg"), sks[0]);
    await createAccount("Telegram", localStorage.getItem("Telegram-reg"), sks[0]);

    localStorage.setItem("validated", "true");
  }

  next("/wallet");
};

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: () => import("@/views/index.vue"),
    meta: { check: false },
  },

  {
    path: "/app",
    component: () => import("@/views/app.vue"),
    meta: { check: false },

    children: [
      {
        // 匹配根路径
        path: "/app",
        // 重定向到
        redirect: "wallet",
      },

      {
        path: "/wallet",
        component: () => import("@/views/wallet.vue"),
        meta: { check: false },
      },

      {
        path: "/transfer",
        component: () => import("@/views/transfer.vue"),
        meta: { check: false },
      },
    ],
  },

  {
    name: "error",
    path: "/:path(.*)*",
    component: () => import("@/views/error.vue"),
    meta: { check: false },
  },

  {
    path: "/omniauth/:provider/callback",
    name: "Callback",
    component: () => import("@/views/wallet.vue"),
    beforeEnter: callbackOmniAuth,
  },

  {
    path: "/omniauth/:provider/registercallback",
    name: "Callback1",
    component: () => import("@/views/wallet.vue"),
    beforeEnter: registerCallbackOmniAuth,
  },

  {
    path: "/omniauth/:provider/logincallback",
    name: "Callback2",
    component: () => import("@/views/wallet.vue"),
    beforeEnter: loginCallbackOmniAuth,
  },
];

const router = createRouter({
  // History模式
  history: createWebHistory(),

  scrollBehavior: () => ({
    behavior: "smooth",
    top: 0,
  }),

  routes,
});

router.beforeEach(({ meta: { check, home }, name, params }, from, next) => {
  if (check) {
    const { account } = useWallet().connect;
    if (account) return next();
    return next("/");
  }

  return next();
});

export default router;
