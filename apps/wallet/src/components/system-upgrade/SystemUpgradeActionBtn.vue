<template>
  <ActionBtn
    v-model="upgradeModel"
    :text="btnText"
    :title="$t('app.software_update')"
    size="default"
    :variant="!isMobileHighlight ? 'outlined' : 'text'"
    density="comfortable"
    :icon="isMobileHighlight ? mdiCloudDownload : false"
    :append-icon="isHighlightedAction && !isMobileHighlight ? mdiCloudDownload : undefined"
    :color="isHighlightedAction ? 'warning' : undefined"
    :rounded="isHighlightedAction ? true : undefined"
    :submit="form => submitUpgrade(form.modelValue as SystemUpgradeFormProps['modelValue'])"
    data-test-id="submit-upgrade-btn"
    @opened="emit('editing', true)"
    @closed="onClosed"
    @failed="useOnFailedOperation"
    @submitted="useOnSuccessfulOperation"
  >
    <template #default="{ model: elem }">
      <SystemUpgradeForm
        v-show="screen === SystemUpgradeScreen.Form"
        :mode="formMode"
        :model-value="elem.value.modelValue as SystemUpgradeFormProps['modelValue']"
        @update:model-value="elem.value.modelValue = $event"
        @valid="elem.value.valid = $event"
        @loading="formLoading = $event"
        @submit="goToConfirmation(elem.value.modelValue)"
      />

      <SystemUpgradeConfirmationScreen
        v-if="screen === SystemUpgradeScreen.Confirm"
        :wasm-module-checksum="wasmChecksum"
        :comment="elem.value.modelValue.comment"
        @update:comment="
          elem.value.modelValue = {
            ...elem.value.modelValue,
            comment: $event,
          }
        "
      />
    </template>
    <template #actions="{ submit, loading: saving, model: elem }">
      <VBtn
        v-if="screen === SystemUpgradeScreen.Form"
        :disabled="saving"
        :append-icon="formMode === SystemUpgradeFormMode.Advanced ? mdiCloudDownload : mdiWrenchCog"
        variant="text"
        data-test-id="toggle-form-mode-btn"
        @click="toggleFormMode"
      >
        {{
          formMode === SystemUpgradeFormMode.Advanced ? $t('terms.automated') : $t('terms.advanced')
        }}
      </VBtn>
      <VSpacer />
      <div class="d-flex align-md-center justify-end flex-column-reverse flex-md-row ga-2">
        <VBtn
          v-if="screen === SystemUpgradeScreen.Form && formMode === SystemUpgradeFormMode.Registry"
          :disabled="station.versionManagement.loading || formLoading"
          color="primary"
          variant="text"
          :append-icon="mdiRefresh"
          size="small"
          @click="station.checkVersionUpdates"
        >
          {{ $t('app.check_updates_btn') }}
        </VBtn>
        <VBtn
          v-if="screen === SystemUpgradeScreen.Form"
          :loading="saving"
          :disabled="!elem.value.valid"
          color="primary"
          variant="flat"
          data-test-id="continue-action-btn"
          @click="goToConfirmation(elem.value.modelValue)"
        >
          {{ $t('terms.continue') }}
        </VBtn>
        <VBtn
          v-else-if="screen === SystemUpgradeScreen.Confirm"
          :loading="saving"
          :disabled="saving"
          color="primary"
          variant="flat"
          data-test-id="submit-action-btn"
          @click="submit"
        >
          {{ $t('terms.submit') }}
        </VBtn>
      </div>
    </template>
  </ActionBtn>
</template>

<script lang="ts" setup>
import { mdiCloudDownload, mdiRefresh, mdiWrenchCog } from '@mdi/js';
import { computed, Ref, ref } from 'vue';
import { VBtn } from 'vuetify/components';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import SystemUpgradeForm, { SystemUpgradeFormProps } from './SystemUpgradeForm.vue';
import { useDefaultUpgradeModel } from '~/composables/system-upgrade.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Request } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import {
  arrayBufferToHashHex,
  arrayBufferToHex,
  hexStringToArrayBuffer,
} from '~/utils/crypto.utils';
import { assertAndReturn, toUint8Array } from '~/utils/helper.utils';
import { SystemUpgradeFormMode, SystemUpgradeScreen } from './system-upgrade.types';
import SystemUpgradeConfirmationScreen from './SystemUpgradeConfirmationScreen.vue';
import { useAppStore } from '~/stores/app.store';
import { useI18n } from 'vue-i18n';

const props = withDefaults(
  defineProps<{
    mode?: 'default' | 'highlight';
  }>(),
  {
    mode: 'default',
  },
);

const i18n = useI18n();
const app = useAppStore();
const isHighlightedAction = computed(() => props.mode === 'highlight');
const isMobileHighlight = computed(() => isHighlightedAction.value && app.isMobile);
const btnText = computed(() => {
  if (isMobileHighlight.value) {
    return undefined;
  }

  return isHighlightedAction.value ? i18n.t('terms.update') : i18n.t('app.software_update');
});

const station = useStationStore();
const upgradeModel = ref(useDefaultUpgradeModel()) as Ref<SystemUpgradeFormProps>;
const screen = ref<SystemUpgradeScreen>(SystemUpgradeScreen.Form);
const formMode = ref<SystemUpgradeFormMode>(SystemUpgradeFormMode.Registry);
const toggleFormMode = () => {
  upgradeModel.value = useDefaultUpgradeModel();
  formMode.value =
    formMode.value === SystemUpgradeFormMode.Advanced
      ? SystemUpgradeFormMode.Registry
      : SystemUpgradeFormMode.Advanced;
};
const wasmChecksum = ref<string>('');
const formLoading = ref(false);
const goToConfirmation = async (model: SystemUpgradeFormProps['modelValue']): Promise<void> => {
  if (!model.wasmModuleExtraChunks) {
    const wasmModule = assertAndReturn(model.wasmModule, 'model.wasmModule is required');
    wasmChecksum.value = await arrayBufferToHashHex(wasmModule);
  } else {
    wasmChecksum.value = arrayBufferToHex(
      toUint8Array(model.wasmModuleExtraChunks.wasm_module_hash),
    );
  }

  screen.value = SystemUpgradeScreen.Confirm;
};

const submitUpgrade = async (model: SystemUpgradeFormProps['modelValue']): Promise<Request> => {
  let wasmModule = new Uint8Array();
  if (!model.wasmModuleExtraChunks) {
    wasmModule = new Uint8Array(assertAndReturn(model.wasmModule, 'model.wasmModule is required'));
  } else if (model.wasmModule) {
    wasmModule = new Uint8Array(model.wasmModule);
  }

  const res = await station.service.systemUpgrade(
    {
      arg:
        model.wasmInitArg && model.wasmInitArg.length > 0
          ? [new Uint8Array(hexStringToArrayBuffer(model.wasmInitArg))]
          : [],
      module: wasmModule,
      module_extra_chunks: model.wasmModuleExtraChunks ? [model.wasmModuleExtraChunks] : [],
      target: assertAndReturn(model.target, 'model.target is required'),
      take_backup_snapshot: [true],
    },
    {
      comment: model.comment,
    },
  );

  // Refresh the version update status after the upgrade
  station.checkVersionUpdates();

  return res;
};

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

const onClosed = () => {
  formMode.value = SystemUpgradeFormMode.Registry;
  screen.value = SystemUpgradeScreen.Form;
  upgradeModel.value = useDefaultUpgradeModel();

  emit('editing', false);
};
</script>
