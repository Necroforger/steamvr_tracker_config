<script setup>
import { computed, onMounted, ref } from 'vue';
const settings = ref({
    executable_path: `C:\\Program Files (x86)\\Steam\\steamapps\\common\\SteamVR\\tools\\lighthouse\\bin\\win64\\lighthouse_console.exe`,
})


const device_id    = ref("")
const device_class = ref("")
const device_model = ref("")

const scanning = ref(false)

const backups = ref([])
const file_json = ref("")

import { shell, invoke, fs, path, tauri, window } from '@tauri-apps/api'
import { ElTabs, ElTabPane, ElCard, ElButton, ElInput, ElImage, ElRow, ElContainer, ElButtonGroup, ElForm, ElAlert, ElSpace, ElIcon, ElTooltip, ElDivider, ElCol, ElSelect, ElOption, ElDialog, ElText, ElNotification, ElTable, ElTableColumn, ElScrollbar, ElMessageBox} from 'element-plus'
import * as util from "../Util"
import Loading from './Loading.vue'

const extra_models = ref([
    'generic_tracker',
    'generic_hmd',
    'vr_glove',
])

onMounted( async () => {
    document.addEventListener('DOMContentLoaded', function() {
        setTimeout(() => { 
            tauri.invoke("show_window")
        }, 300)
    })

    await load_settings()
    await load_connected_device()
    await load_backups()
})

async function load_settings() {
    try {
        settings.value = JSON.parse(await fs.readTextFile("steamvr_config_editor.json"))
    } catch (e) {
        console.log("Could not load settings: " + e)
    }
}
async function save_settings() {
    const state = JSON.stringify(settings.value)
    await fs.writeTextFile("steamvr_config_editor.json", state)

    ElNotification({
        type: 'success',
        message: 'saved settings to disk'
    })
}

async function load_connected_device() {
    try {
        scanning.value = true
        let config = await util.get_lighthouse_config(settings.value.executable_path)
        console.log(config)
        file_json.value = config
        apply_json(config)
    

        ElNotification({
            title: "Discovered Device",
            message: device_id.value,
            type: "info",
        })
    } catch (error) {
        ElNotification({
            title: "Scan Error",
            message: "failed to discover a connected device. Try again",
            type: "error",
            duration: 3000,
        })
    } finally {
        scanning.value = false
    }
}

/**
 * Overlay your personal changes to the original config values.
 * @returns { string }
 */
function overlayed_json() {
    let parsed = JSON.parse(file_json.value)
    
    delete parsed['model_number']
    parsed['model_name']              = 'Vive Tracker PVT'
    parsed['render_model']            = device_model.value
    parsed['tracked_controller_role'] = ""
    parsed['device_class']            = "generic_tracker"

    return JSON.stringify(parsed)
}

/**
 * Upload the configuration settings to the connected device.
 */
async function upload_to_connected_device() {
    if (device_id.value == "") {
        ElNotification({
            message: "You have no devices loaded",
            type: 'error',
        })
        return
    }
    const payload = overlayed_json()
    if (!util.backup_exists(device_id.value)) {
        await backup()
    }

    try {
        console.log(`Invoking with args: ${settings.value.executable_path},  ${payload}`)
        let result = await util.upload_lighthouse_config(settings.value.executable_path, payload)
        ElMessageBox.alert(result, 'Execution Result', {
            closeOnClickModal: true,
        })
    } catch(e) {
        ElNotification({
            title: "Error uploading config",
            message: e,
            type: 'error'
        })
    }
}

async function load_backups() {
    let b = await fs.readDir("backups")
    backups.value = b
}

/**
 * Load the given JSON data into the form
 * @param {string} js 
 */
async function apply_json(js) {
    const parsed = JSON.parse(js)
    device_id.value = parsed['device_serial_number']
    device_class.value = parsed['device_class']
    device_model.value = parsed['render_model']
}

async function clear() {
    device_id.value       = ""
    device_class.value    = ""
    device_model.value    = ""
    file_json.value       = ""
}

// Restore device to original configuration
/**
 * 
 * @param {string} serial_number device id
 */
async function restore_config(serial_number) {
    serial_number = serial_number || device_id.value
    if (serial_number == "") {
        ElNotification({
            message: "You need to have a device in view",
            type: 'warning'
        })
        return
    }
    if (!(await util.backup_exists(serial_number))) {
        ElNotification({
            message: "You do not have any backups saved for this serial number",
            type: 'error',
        })
        return
    }
    const res = await util.upload_lighthouse_config(settings.value.executable_path, await util.read_backup(serial_number))
    ElMessageBox.alert(res, "Restoration Log")
}

async function restore_config_advanced(serial_number) {
    try {
        await ElMessageBox.confirm(
                    "Are you absolutely certain you know what you're doing? Your serial number will be changed if it does not match the original", 
                     "Dangerous waters ahead")
        await restore_config(serial_number)
    } catch(e) {
        ElNotification({
            message: "Restore cancelled",
        })
    }
}

async function backup() {
    if (device_id.value == "") {
        ElNotification({message: "You have no devices in view", title: "error"})
        return
    }
    if (await util.backup_exists(device_id.value)) {
        ElNotification({
            message: "A backup for this serial number already exists",
        })
        return
    }
    await util.write_backup(device_id.value, file_json.value)
    ElNotification({
        message: "You saved a backup of " + device_id.value
    })
}
</script>

<template>
    <ElTabs :tab-position="'left'">
        <ElTabPane label="Device">
            <ElSpace direction="vertical" size="100" :fill="true" style="width: 100%">
                <Loading :is_loading="scanning"></Loading>
                <ElAlert :closable="false" type="warning" title="Note: Unplug all other steamvr devices except the one you wish to configure. Click the search button to load device info."></ElAlert>
                <!-- Action buttons -->
                <ElButtonGroup >
                    <ElTooltip content="reload (search for devices)" placement="top" effect="light">
                        <ElButton @click="load_connected_device" type=""><ElIcon><Refresh/></ElIcon>search</ElButton>
                    </ElTooltip>
                    <ElTooltip content="clear targeted device" placement="top" effect="light">
                        <ElButton @click="clear" type=""><el-icon><DeleteFilled /></el-icon> clear</ElButton>
                    </ElTooltip>
                    <ElTooltip effect="light" placement="top" content="save the json contents to a backup file so it can be restored later">
                        <ElButton @click="backup"><el-icon><Bottom /></el-icon>Backup</ElButton>
                    </ElTooltip>
                    <ElTooltip effect="light" placement="top" content="Restore your device to its stock configuration. Requires a previously saved backup">
                        <ElButton @click="restore_config()"><el-icon><DArrowLeft /></el-icon>Restore</ElButton>
                    </ElTooltip>


                </ElButtonGroup>
                <!-- /Action buttons -->

                <!-- Device -->
                <ElCard>
                    <template v-slot:header>
                        <el-icon><Smoking /></el-icon>{{ " " + device_id }}
                    </template>
                    <ElSpace direction="vertical" :fill="true" style="width: 100%">
                        <!-- Class options -->
                        <ElRow :align="'bottom'" :gutter="1">
                            <ElCol :span="4"><label>class: </label></ElCol>
                            <ElCol :span="8"><ElSelect disabled v-model="device_class" :allow-create="true" :filterable="true">
                                <ElOption label="generic_tracker" value="generic_tracker"></ElOption>
                                <ElOption label="hmd" value="hmd"></ElOption>
                                <ElOption label="controller" value="controller"></ElOption>
                            </ElSelect></ElCol>
                        </ElRow>
                        <!-- /Class options -->

                        <!-- Model Options -->
                        <ElRow :align="'bottom'" :gutter="1">
                            <ElCol :span="4"><label>model: </label></ElCol>
                            <ElCol :span="8">
                                <ElSelect v-model="device_model" :allow-create="true" :filterable="true">
                                    <ElOption value="vr_tracker_vive_1_0"    name="vr_tracker_vive_1_0" />
                                    <ElOption value="vr_controller_vive_1_5" name="vr_controller_vive_1_5" />
                                    <template v-for="model in extra_models">
                                        <ElOption :label="model" :value="model"></ElOption>
                                    </template>
                                </ElSelect>
                            </ElCol>
                        </ElRow>
                        <!-- /Model options -->

                        <ElDivider></ElDivider>
                        <ElButton @click="upload_to_connected_device" type="primary">Configure as Tracker</ElButton>
                    </ElSpace>
                </ElCard> 
                <!-- / Device -->

                <!-- JSON -->
                <ElDivider></ElDivider>
                <ElCard header="JSON View">
                    <ElScrollbar :height="300">
                        <ElText> {{ file_json}}</ElText>
                    </ElScrollbar>
                </ElCard>
                <!-- /Json -->
            </ElSpace>
        </ElTabPane>

        <ElTabPane label="Backups">
            <ElCard>
                <ElButton @click="load_backups">Refresh</ElButton>
                <table>
                    <th>
                        <td>Serial Number</td>
                        <td>Operations</td>
                    </th>
                    <tbody>
                        <tr v-for="file in backups">
                            <td>{{ file.name }}</td>
                            <td>
                                <ElButton type="success" @click="restore_config_advanced(file.name)">Restore</ElButton>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </ElCard>
        </ElTabPane>

        <!-- Settings -->
        <ElTabPane label="Settings" name="Settings">
            <ElCard>
                <p>Lighthouse Executable</p>
                <ElInput placeholder="Path to lighthouse_debug.exe" v-model="settings.executable_path"/>
                <ElDivider></ElDivider>
                <ElButton @click="save_settings" type="primary">Save</ElButton>                    
            </ElCard>
        </ElTabPane>
        <!-- /Settings -->

        <ElTabPane label="About" name="about">
            <h1>Steamvr device configurator</h1>
            <ElCard>
                <img src="https://i.pinimg.com/736x/e6/53/66/e65366beffb5f7641a204efa0327700a.jpg" style="width: 150px"/>
                <p>Another questionably useful application brought to you by Nekurou.</p>
            </ElCard>           
            <ElDivider></ElDivider>
            <ElCard title="Overview">
                <p>1. Ensure that only the device you wish to modify is connected to your computer</p>
                <p>2. Click `search` button to query for device information.</p>
                <p>3. click `Configure as Tracker` to apply the new configuration to the device. A backup of the original will be stored locally.</p>
                <p>To restore your device's default configuration press the `restore` button.</p>
                <p>Technical assistance available in the group ERP server</p>
            </ElCard>
        </ElTabPane>
    </ElTabs>
</template>