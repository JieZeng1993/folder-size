<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {open as folder_open} from "@tauri-apps/api/dialog"
import {invoke} from "@tauri-apps/api/tauri";
import {ref} from "vue";
import {ElMessageBox, TableV2FixedDir} from "element-plus";
import {ArrowUpBold} from "@element-plus/icons-vue";

async function selectFile() {
  let file = await folder_open({directory: true})
  await list_file(file)
}

async function rowClick(event: any) {
  await list_file(event.rowData.file_path)
}

async function fileChangeToParent() {
  let getFileParentPathRes: any = await invoke("get_file_parent_path", {folderPath: selectedFile.value});
  if (!getFileParentPathRes.success) {
    ElMessageBox.alert(getFileParentPathRes.msg, "错误")
    return
  }
  await list_file(getFileParentPathRes.data)
}

async function list_file(file: any) {
  selectedFile.value = file;
  let listFileRes: any = await invoke("list_file", {folderPath: file});
  if (!listFileRes.success) {
    ElMessageBox.alert(listFileRes.msg, "错误")
    file_list.value = []
    return;
  }
  let subFiles = listFileRes.data;
  subFiles.sort((a1: any, a2: any) => a2.file_size - a1.file_size)
  for (let subFile of subFiles) {
    subFile.file_size_str = humanFileSize(subFile.file_size);
  }
  file_list.value = subFiles;
}

function humanFileSize(size: number) {
  if (size < 1) {
    return "0";
  }
  let i = Math.floor(Math.log(size) / Math.log(1024));
  let val = (size / Math.pow(1024, i)).toFixed(2);
  return val + ' ' + ['B', 'kB', 'MB', 'GB', 'TB'][i];
}

const file_columns = [{key: "file_path", dataKey: "file_path", title: "文件路径", width: 800}, {
    key: "file_size_str",
    dataKey: "file_size_str",
    title: "文件大小",
    width: 160,
  fixed: TableV2FixedDir.RIGHT
}];
const file_list = ref([]);
const selectedFile = ref("");

</script>

<template>
  <div class="container">
    <el-row>
      <el-col span="24">
        <el-button type="primary" @click="selectFile" size="small">选择文件</el-button>
        已选择路径：
        <el-input style="display: inline" v-model="selectedFile" @keyup.enter="list_file(selectedFile)"></el-input>
        <el-icon v-if="selectedFile" @click="fileChangeToParent">
          <ArrowUpBold/>
        </el-icon>
      </el-col>
    </el-row>

    <el-auto-resizer style="flex: 1">
      <template #default="{ height, width }">
        <el-table-v2
            :columns="file_columns"
            :data="file_list"
            :height="height"
            :width="width"
            :row-event-handlers="{onClick:rowClick}"
            fixed
        >
          <el-table-column prop="address" label="Address"/>
        </el-table-v2>
      </template>
    </el-auto-resizer>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 95vh; /* 或者使用具体的px值，确保父容器高度已设定 */
  overflow: hidden;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
