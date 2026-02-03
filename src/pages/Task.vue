<template>
  <div
    class="min-h-screen bg-base-200 p-4"
    v-if="!showWorkflow && !backTransition"
  >
    <Transition name="t-b">
      <div v-if="!startTransition">
        <!-- 页面标题和统计 -->
        <div class="mb-6">
          <h1 class="text-3xl font-bold text-base-content mb-4">
            自动化工作流仪表盘
          </h1>

          <!-- 统计卡片 -->
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            <div class="stat bg-base-100 rounded-lg shadow">
              <div class="stat-figure text-primary">
                <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <div class="stat-title">运行中</div>
              <div class="stat-value text-primary">3</div>
              <div class="stat-desc">正在执行的任务</div>
            </div>

            <div class="stat bg-base-100 rounded-lg shadow">
              <div class="stat-figure text-success">
                <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                  <path
                    fill-rule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  />
                </svg>
              </div>
              <div class="stat-title">已完成</div>
              <div class="stat-value text-success">12</div>
              <div class="stat-desc">本周完成的任务</div>
            </div>

            <div class="stat bg-base-100 rounded-lg shadow">
              <div class="stat-figure text-error">
                <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                  <path
                    fill-rule="evenodd"
                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
                  />
                </svg>
              </div>
              <div class="stat-title">失败</div>
              <div class="stat-value text-error">2</div>
              <div class="stat-desc">需要处理的错误</div>
            </div>

            <div class="stat bg-base-100 rounded-lg shadow">
              <div class="stat-figure text-info">
                <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" />
                  <path
                    fill-rule="evenodd"
                    d="M4 5a2 2 0 012-2v1a1 1 0 001 1h6a1 1 0 001-1V3a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3z"
                  />
                </svg>
              </div>
              <div class="stat-title">总任务</div>
              <div class="stat-value text-info">8</div>
              <div class="stat-desc">工作流总数</div>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex flex-wrap gap-2 mb-6">
          <button @click="startTransition = true" class="btn btn-primary">
            <svg
              class="w-5 h-5 mr-2"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 6v6m0 0v6m0-6h6m-6 0H6"
              />
            </svg>
            新建工作流
          </button>
          <button class="btn btn-outline">
            <svg
              class="w-5 h-5 mr-2"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
            刷新全部
          </button>
          <button class="btn btn-ghost">
            <svg
              class="w-5 h-5 mr-2"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.207A1 1 0 013 6.5V4z"
              />
            </svg>
            筛选
          </button>
        </div>
      </div>
    </Transition>
    <Transition name="b-t">
      <div v-if="!startTransition">
        <!-- 任务列表 -->
        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
          <!-- 任务卡片 1: 数据备份流程 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">数据备份流程</h2>
                <div class="badge badge-success">运行中</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                每日自动备份数据库和重要文件
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>执行进度</span>
                  <span>75%</span>
                </div>
                <progress
                  class="progress progress-success w-full"
                  value="75"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">定时触发</div>
                <div class="badge badge-outline badge-sm">数据处理</div>
                <div class="badge badge-outline badge-sm">通知</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">查看日志</button>
                <button class="btn btn-primary btn-sm">编辑</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 2: 邮件营销推送 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">邮件营销推送</h2>
                <div class="badge badge-primary">运行中</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                向客户发送个性化营销邮件
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>发送进度</span>
                  <span>1,250/2,000</span>
                </div>
                <progress
                  class="progress progress-primary w-full"
                  value="62"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">API触发</div>
                <div class="badge badge-outline badge-sm">邮件发送</div>
                <div class="badge badge-outline badge-sm">统计分析</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">暂停</button>
                <button class="btn btn-warning btn-sm">停止</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 3: 网站监控检查 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">网站监控检查</h2>
                <div class="badge badge-accent">运行中</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                监控网站可用性和响应时间
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>检查站点</span>
                  <span>8/12</span>
                </div>
                <progress
                  class="progress progress-accent w-full"
                  value="67"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">定时执行</div>
                <div class="badge badge-outline badge-sm">HTTP检查</div>
                <div class="badge badge-outline badge-sm">报警</div>
              </div>

              <div class="text-xs text-base-content/60 mb-4">
                <div>执行间隔：每5分钟</div>
                <div>上次异常：无</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">查看报告</button>
                <button class="btn btn-primary btn-sm">配置</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 4: 用户数据同步 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">用户数据同步</h2>
                <div class="badge badge-success">已完成</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                同步用户数据到分析系统
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>同步记录</span>
                  <span>5,678条</span>
                </div>
                <progress
                  class="progress progress-success w-full"
                  value="100"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">数据库</div>
                <div class="badge badge-outline badge-sm">API同步</div>
                <div class="badge badge-outline badge-sm">验证</div>
              </div>

              <div class="text-xs text-base-content/60 mb-4">
                <div>完成时间：2024-01-15 12:30</div>
                <div>耗时：2分35秒</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">查看结果</button>
                <button class="btn btn-primary btn-sm">重新运行</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 5: 报表生成 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">报表生成</h2>
                <div class="badge badge-success">已完成</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                生成每日销售和用户行为报表
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>生成报表</span>
                  <span>3个文件</span>
                </div>
                <progress
                  class="progress progress-success w-full"
                  value="100"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">数据聚合</div>
                <div class="badge badge-outline badge-sm">图表生成</div>
                <div class="badge badge-outline badge-sm">邮件发送</div>
              </div>

              <div class="text-xs text-base-content/60 mb-4">
                <div>完成时间：2024-01-15 08:15</div>
                <div>文件大小：2.3MB</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">下载报表</button>
                <button class="btn btn-primary btn-sm">查看详情</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 6: 库存警报 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">库存警报</h2>
                <div class="badge badge-error">失败</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                监控库存并发送低库存警报
              </p>

              <div class="mb-4">
                <div class="alert alert-error">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path
                      fill-rule="evenodd"
                      d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                    />
                  </svg>
                  <span class="text-xs">API连接超时</span>
                </div>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">库存检查</div>
                <div class="badge badge-outline badge-sm">阈值监控</div>
                <div class="badge badge-outline badge-sm">通知推送</div>
              </div>

              <div class="text-xs text-base-content/60 mb-4">
                <div>失败时间：2024-01-15 10:22</div>
                <div>重试次数：3/3</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">查看错误</button>
                <button class="btn btn-warning btn-sm">重试</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 7: 社交媒体发布 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">社交媒体发布</h2>
                <div class="badge badge-warning">待执行</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                定时发布内容到多个社交平台
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>计划发布</span>
                  <span>5个平台</span>
                </div>
                <progress
                  class="progress progress-warning w-full"
                  value="0"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">内容管理</div>
                <div class="badge badge-outline badge-sm">多平台</div>
                <div class="badge badge-outline badge-sm">定时发布</div>
              </div>

              <div class="text-xs text-base-content/60 mb-4">
                <div>计划时间：2024-01-15 18:00</div>
                <div>目标平台：微博、抖音、小红书...</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">预览内容</button>
                <button class="btn btn-primary btn-sm">立即执行</button>
              </div>
            </div>
          </div>

          <!-- 任务卡片 8: 系统清理 -->
          <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
              <div class="flex items-center justify-between mb-2">
                <h2 class="card-title text-lg">系统清理</h2>
                <div class="badge badge-info">已暂停</div>
              </div>
              <p class="text-sm text-base-content/70 mb-3">
                清理临时文件和过期日志
              </p>

              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>已清理</span>
                  <span>2.1GB</span>
                </div>
                <progress
                  class="progress progress-info w-full"
                  value="45"
                  max="100"
                ></progress>
              </div>

              <div class="flex flex-wrap gap-2 mb-4">
                <div class="badge badge-outline badge-sm">文件清理</div>
                <div class="badge badge-outline badge-sm">日志轮转</div>
                <div class="badge badge-outline badge-sm">磁盘优化</div>
              </div>

              <div class="text-xs text-base-content/60 mb-4">
                <div>暂停时间：2024-01-15 11:15</div>
                <div>预计释放：4.7GB</div>
              </div>

              <div class="card-actions justify-end">
                <button class="btn btn-ghost btn-sm">查看详情</button>
                <button class="btn btn-success btn-sm">继续</button>
              </div>
            </div>
          </div>
        </div>

        <!-- 底部操作区域 -->
        <div class="mt-8 flex justify-center">
          <div class="join">
            <button class="join-item btn btn-outline">上一页</button>
            <button class="join-item btn btn-outline btn-active">1</button>
            <button class="join-item btn btn-outline">2</button>
            <button class="join-item btn btn-outline">3</button>
            <button class="join-item btn btn-outline">下一页</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
  <Workflow v-model:show="showWorkflow"></Workflow>
</template>

<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import {
  nextTick,
  onBeforeMount,
  onBeforeUnmount,
  ref,
  Transition,
  watchEffect,
} from "vue";
import Workflow from "./Workflow.vue";
import { onBack, onForward } from "../views/AppView.vue";

const startTransition = ref(false);
const backTransition = ref(false);
const showWorkflow = ref(false);

watchEffect(() => {
  if (startTransition.value) {
    setTimeout(() => {
      showWorkflow.value = true;
    }, 300);
  }
});

onBack((back) => {
  if (showWorkflow.value) {
    showWorkflow.value = false;
    backTransition.value = true;
    setTimeout(() => {
      backTransition.value = false;
      nextTick(() => {
        startTransition.value = false;
      });
    }, 300);
  } else {
    back();
  }
});

onBeforeMount(async () => {
  await getCurrentWindow().setSize(new LogicalSize(1200, 900));
  await getCurrentWindow().center();
});

onBeforeUnmount(async () => {
  await getCurrentWindow().setSize(new LogicalSize(800, 600));
  await getCurrentWindow().center();
});
</script>
