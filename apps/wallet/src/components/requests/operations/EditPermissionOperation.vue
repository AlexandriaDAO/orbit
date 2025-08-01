<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="operation.input.resource">
      <template #name>{{ $t('terms.resource') }}</template>
      <template #content>
        {{
          $t(
            `permissions.resources.${fromResourceToResourceEnum(operation.input.resource).toLowerCase()}`,
          )
        }}
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" indeterminate />

  <template v-else>
    <VAlert
      v-if="currentPermissionFailed"
      type="error"
      variant="tonal"
      density="compact"
      class="mb-4"
    >
      {{ $t('requests.failed_to_fetch_details') }}
      <div>{{ currentPermissionFailed }}</div>
    </VAlert>

    <PermissionItemForm
      v-if="permission.allow && permission.resource"
      :model-value="permission.allow"
      :resource="permission.resource"
      :current-permission="currentPermission"
      readonly
      class="py-2"
    />
  </template>
</template>

<script setup lang="ts">
import { computed, onBeforeMount, Ref, ref } from 'vue';
import { VProgressCircular } from 'vuetify/components';
import PermissionItemForm from '~/components/permissions/PermissionItemForm.vue';
import logger from '~/core/logger.core';
import { EditPermissionOperation, Permission, Request } from '~/generated/station/station.did';
import { fromResourceToResourceEnum } from '~/mappers/permissions.mapper';
import { useStationStore } from '~/stores/station.store';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { useAppStore } from '~/stores/app.store';
import { variantIs } from '~/utils/helper.utils';
import { getErrorMessage } from '~/utils/error.utils';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditPermissionOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const isDiffMode = computed(() => !isListMode.value && variantIs(props.request.status, 'Created'));
const station = useStationStore();
const appStore = useAppStore();
const permission: Ref<Partial<Permission>> = ref({});
const currentPermission: Ref<Permission | undefined> = ref();
const currentPermissionFailed = ref<string | undefined>();
const loading = ref(false);

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }
    loading.value = true;

    const { permission: result } = await station.service.getPermission({
      resource: props.operation.input.resource,
    });

    if (isDiffMode.value) {
      // snapshot original for diff
      currentPermission.value = { ...result, allow: { ...result.allow } };
    }
    // merge overrides for updated view
    const updatedAllow = { ...result.allow };
    updatedAllow.auth_scope = props.operation.input.auth_scope?.[0] ?? updatedAllow.auth_scope;
    updatedAllow.users = props.operation.input.users?.[0] ?? updatedAllow.users;
    updatedAllow.user_groups = props.operation.input.user_groups?.[0] ?? updatedAllow.user_groups;
    permission.value = { resource: result.resource, allow: updatedAllow };
  } catch (e) {
    logger.error('Failed to fetch permission details', e);
    appStore.sendErrorNotification(e);
    currentPermissionFailed.value = getErrorMessage(e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  fetchDetails();
});
</script>
