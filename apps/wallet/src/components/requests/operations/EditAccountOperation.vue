<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="props.operation.input.name?.[0]">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ props.operation.input.name[0] ?? '-' }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="props.operation.input.change_assets?.[0]">
      <template #name>{{ $t('terms.assets') }}</template>
      <template #content>
        <div v-if="editAssets.addAssets">
          {{ editAssets.addAssets }}
        </div>

        <div v-if="editAssets.removeAssets">
          {{ editAssets.removeAssets }}
        </div>

        <div v-if="editAssets.replaceAssets">
          {{ editAssets.replaceAssets }}
        </div>
      </template>
    </RequestOperationListRow>
  </div>
  <LoadingMessage v-else-if="loading" />

  <template v-else>
    <VAlert
      v-if="currentAccountModelFailed"
      type="error"
      variant="tonal"
      density="compact"
      class="mb-4"
    >
      {{ $t('requests.failed_to_fetch_details') }}
      <div>{{ currentAccountModelFailed }}</div>
    </VAlert>

    <AccountSetupWizard :model-value="model" mode="view" :current-model="currentAccountModel" />
  </template>
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountSetupWizard, {
  AccountSetupWizardModel,
  CurrentAccountSetupWizardModel,
} from '~/components/accounts/wizard/AccountSetupWizard.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import {
  useDefaultAccountSetupWizardModel,
  useLoadAccountSetupWizardModel,
} from '~/composables/account.composable';
import logger from '~/core/logger.core';
import { EditAccountOperation, Request } from '~/generated/station/station.did';
import { deepClone, unreachable, variantIs } from '~/utils/helper.utils';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { useI18n } from 'vue-i18n';
import { useStationStore } from '~/stores/station.store';
import { getErrorMessage } from '~/utils/error.utils';
import { useAppStore } from '~/stores/app.store';
const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditAccountOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();
const appStore = useAppStore();

const isListMode = computed(() => props.mode === 'list');
const isDiffMode = computed(() => !isListMode.value && variantIs(props.request.status, 'Created'));
const model: Ref<AccountSetupWizardModel> = ref(useDefaultAccountSetupWizardModel());
const loading = ref(false);

const currentAccountModel = ref<CurrentAccountSetupWizardModel | undefined>(undefined);
const currentAccountModelFailed = ref<string | undefined>();
const editAssets = computed(() => {
  const assets = {
    addAssets: '',
    replaceAssets: '',
    removeAssets: '',
  };
  if (props.operation.input.change_assets[0]) {
    if (variantIs(props.operation.input.change_assets[0], 'Change')) {
      if (props.operation.input.change_assets[0].Change.add_assets.length > 0) {
        assets.addAssets = `${i18n.t('requests.types.editaccount.added_assets')}: ${assetIdsToString(
          props.operation.input.change_assets[0].Change.add_assets,
        )}`;
      }

      if (props.operation.input.change_assets[0].Change.remove_assets.length > 0) {
        assets.removeAssets = `${i18n.t('requests.types.editaccount.removed_assets')}: ${assetIdsToString(
          props.operation.input.change_assets[0].Change.remove_assets,
        )}`;
      }
    } else if (variantIs(props.operation.input.change_assets[0], 'ReplaceWith')) {
      assets.replaceAssets = `${i18n.t('requests.types.editaccount.replaced_assets')}: ${assetIdsToString(
        props.operation.input.change_assets[0].ReplaceWith.assets,
      )}`;
    } else {
      unreachable(props.operation.input.change_assets[0]);
    }
  }

  return assets;
});

const station = useStationStore();

function assetIdsToString(ids: string[]): string {
  return ids
    .map(id => {
      const maybeAsset = station.configuration.details.supported_assets.find(
        asset => asset.id == id,
      );
      if (maybeAsset) {
        return `${maybeAsset.symbol} (${maybeAsset.name})`;
      } else {
        return id;
      }
    })
    .join(', ');
}

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentModel = await useLoadAccountSetupWizardModel(props.operation.input.account_id);

    model.value = deepClone(currentModel);

    if (isDiffMode.value) {
      // make copy of currentModel to avoid mutating the original
      currentAccountModel.value = deepClone(currentModel);
    }

    if (props.operation.input.name?.[0]) {
      model.value.configuration.name = props.operation.input.name[0];
    }

    if (props.operation.input.configs_request_policy?.[0]) {
      model.value.request_policy.configurationRule = variantIs(
        props.operation.input.configs_request_policy[0],
        'Remove',
      )
        ? undefined
        : props.operation.input.configs_request_policy[0].Set;
    }

    if (props.operation.input.transfer_request_policy?.[0]) {
      model.value.request_policy.transferRule = variantIs(
        props.operation.input.transfer_request_policy[0],
        'Remove',
      )
        ? undefined
        : props.operation.input.transfer_request_policy[0].Set;
    }

    if (props.operation.input.read_permission?.[0]) {
      model.value.permission.read = props.operation.input.read_permission?.[0];
    }

    if (props.operation.input.transfer_permission?.[0]) {
      model.value.permission.transfer = props.operation.input.transfer_permission?.[0];
    }

    if (props.operation.input.configs_permission?.[0]) {
      model.value.permission.configuration = props.operation.input.configs_permission?.[0];
    }

    if (props.operation.input.change_assets?.[0]) {
      if (variantIs(props.operation.input.change_assets[0], 'Change')) {
        const { add_assets, remove_assets } = props.operation.input.change_assets[0].Change;

        model.value.configuration.assets = [
          ...add_assets,
          ...(model.value.configuration.assets ?? []),
        ];

        model.value.configuration.assets = model.value.configuration.assets.filter(
          asset => !remove_assets.includes(asset),
        );
      } else if (variantIs(props.operation.input.change_assets[0], 'ReplaceWith')) {
        model.value.configuration.assets =
          props.operation.input.change_assets[0].ReplaceWith.assets;
      } else {
        unreachable(props.operation.input.change_assets[0]);
      }
    }
  } catch (e) {
    logger.error('Failed to fetch account details', e);
    if (isDiffMode.value) {
      currentAccountModelFailed.value = getErrorMessage(e);
    }
    appStore.sendErrorNotification(e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const model: AccountSetupWizardModel = useDefaultAccountSetupWizardModel();
  model.configuration.id = props.operation.input.account_id;

  fetchDetails();
});
</script>
