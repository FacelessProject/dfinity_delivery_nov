<script setup lang="ts">
import { ref } from "vue";
import { RouteBack, hris, eras } from "@/components/community";
import { useConfig, useWallet, Connect } from "@/store";

let loading = ref(false);

let platforms: any = ref([]);

const Wallet = useWallet();
const config = useConfig();

const emits = defineEmits(["loadOtherComponent"]);

const readHRIPlatforms = async () => {
  return new Promise(res => {
    setTimeout(() => {
      res(hris);
    }, 3000);
  });
};

const getHRIPlatforms = async () => {
  loading.value = true;
  platforms.value = await readHRIPlatforms();
  for (let i in platforms.value) {
    platforms.value[i].validated = window.localStorage.getItem(platforms.value[i].name) !== null;
  }
  loading.value = false;
};

getHRIPlatforms();

const onUserClickRouteBack = (name: string) => {
  emits("loadOtherComponent", { name });
};

const userCreateHRIPlatform = (name: string) => {
  if (name === "Twitter") {
    window.location.href = `https://oauth.faceless.live/auth/twitter?auth_origin_url=${encodeURI("http://127.0.0.1:80")}/omniauth/twitter/logincallback`;
    return;
  }

  if (name === "Google") {
    window.location.href = `https://oauth.faceless.live/auth/google_oauth2?auth_origin_url=${encodeURI("http://127.0.0.1:80")}/omniauth/google_oauth2/logincallback`;
    return;
  }

  if (name === "Telegram") {
    window.Telegram.Login.auth({ bot_id: '6290560220', request_access: 'write', embed: 1 }, (data) => {
      console.log(data, '这是回调数据');//这里的data和之前返回的user数据和格式无差异
      if (!data) {
      //失败时你需要做的逻辑
        window.location.href = `http://127.0.0.1:80/omniauth/cancel/logincallback`;
        return
      }
      
      window.location.href = `http://127.0.0.1:80/omniauth/telegram/logincallback?nickname=${data.username}`;
      return
    });
  }

  emits("loadOtherComponent", { name });
};
</script>

<template>
  <div id="AddHRIPlatform">
    <span class="title">
      Log into 2 HRI platforms <br />
      to recover your accounts
    </span>

    <span class="hint">
      Faceless is an app that brings regulation- <br />
      compliant financial privacy to the mass.
    </span>

    <n-scrollbar trigger="none" style="max-height: 500px">
      <n-spin :show="loading" size="large" description="Loading platforms">
        <div class="content">
          <!-- 平台列表 -->
          <XyzTransitionGroup
            class="transitionContent"
            xyz="fade small-3 down-25% stagger-1.5"
            appear-visible
          >
            <div :class="[`item`, {validated: it.validated}]" v-for="it of platforms" :key="it">
              <n-tooltip>
                <template #trigger>
                  <div class="item_trigger" @click="userCreateHRIPlatform(it.key)">
                    <img :src="it.icon" :alt="it.name" />
                  </div>
                </template>

                {{ it.name }}
              </n-tooltip>
            </div>
          </XyzTransitionGroup>

          <!-- 无数据 -->
          <div class="noData" v-if="!loading && !platforms.length">
            <n-empty description="No platform yet" />
          </div>
        </div>
      </n-spin>
    </n-scrollbar>

    <!-- 返回按钮 -->
    <div class="back">
      <RouteBack name="AccountList" @userClickRouteBack="onUserClickRouteBack" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
#AddHRIPlatform {
  flex: 0;
  display: flex;
  flex-direction: column;

  box-sizing: border-box;

  width: 550px;
  overflow: hidden;
  border-radius: 18px;
  background-color: #1a2736;

  padding: 75px 100px;

  position: relative;

  .title {
    font-size: 32px;
    font-weight: 600;
    color: #ffffff;
    line-height: 38px;
    text-align: center;
    font-family: "JosefinSans-Medium";
  }

  .hint {
    font-size: 17px;
    font-weight: 400;
    margin-top: 16px;
    color: rgba(255, 255, 255, 0.5);
    font-family: "Ubuntu-Regular";
    text-align: center;
    line-height: 24px;
    margin-bottom: 25px;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;

    min-height: 200px;

    margin-top: 25px;

    .transitionContent {
      display: flex;
      align-items: flex-start;
      flex-wrap: wrap;

      .item {
        width: 80px;
        height: 80px;
        border-radius: 10px;
        background: #2b3745;

        overflow: hidden;

        display: flex;
        flex-direction: column;

        margin: 3.75px;
        cursor: pointer;
        transition: all 0.5s;

        &:hover {
          transform: scale(0.96);
          background-color: #127cf8;

          .item_trigger img {
            opacity: 1;
          }
        }

        &:active {
          transform: scale(0.7);
        }

        .item_trigger {
          flex: 1;
          display: flex;
          align-items: center;
          justify-content: center;

          img {
            width: 45px;
            height: 45px;
            object-fit: contain;
            transition: all 0.5s;
            opacity: 0.5;
          }
        }

        &:last-child .item_trigger img {
          width: 35px;
          height: 35px;
        }
      }

      .validated {
        transform: scale(0.96);
        background-color: #007c00;
      }

    }

    .noData {
      flex: 1;
      display: flex;
      align-items: center;
      justify-content: center;
    }
  }

  .back {
    top: 16px;
    left: 16px;
    position: absolute;

    display: flex;
    flex-direction: column;
  }
}
</style>
