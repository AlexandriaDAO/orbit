<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && displayId"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
    <DiffView :before-value="currentEntry?.blockchain" :after-value="model.blockchain">
      <template #default="{ value, diffMode }">
        <BlockchainAutocomplete
          :model-value="value"
          class="mb-2"
          :label="$t('terms.blockchain')"
          :prepend-icon="mdiTransitConnectionVariant"
          :rules="diffMode === 'before' ? [] : [requiredRule]"
          variant="filled"
          density="comfortable"
          :readonly="isViewMode || diffMode === 'before' || !!model.id"
          @update:model-value="val => diffMode === 'after' && (model.blockchain = val)"
        />
      </template>
    </DiffView>
    <DiffView :before-value="currentEntry?.address_owner" :after-value="model.address_owner">
      <template #default="{ value, diffMode }">
        <VTextField
          :model-value="value"
          :name="diffMode === 'before' ? 'address_owner-before' : 'address_owner'"
          :label="$t('terms.name')"
          variant="filled"
          :rules="diffMode === 'before' ? [] : [requiredRule]"
          class="mb-2"
          density="comfortable"
          :prepend-icon="mdiAccount"
          :readonly="isViewMode || diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (model.address_owner = val)"
        />
      </template>
    </DiffView>
    <DiffView :before-value="currentEntry?.address" :after-value="model.address">
      <template #default="{ value, diffMode }">
        <VTextField
          :model-value="value"
          :name="diffMode === 'before' ? 'address-before' : 'address'"
          class="mb-2"
          :label="$t('terms.address')"
          :prepend-icon="mdiKeyChain"
          :rules="diffMode === 'before' ? [] : [requiredRule]"
          variant="filled"
          density="comfortable"
          :readonly="isViewMode || diffMode === 'before' || !!model.id"
          @update:model-value="val => diffMode === 'after' && (model.address = val)"
        />
      </template>
    </DiffView>
    <DiffView
      :before-value="currentEntry?.metadata"
      :after-value="model.metadata"
      :compare-values="compareMetadata"
    >
      <template #default="{ value, diffMode }">
        <MetadataField
          :model-value="value"
          :label="$t('terms.metadata')"
          :readonly="isViewMode || diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (model.metadata = val)"
        />
      </template>
    </DiffView>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiAccount, mdiKeyChain, mdiTransitConnectionVariant } from '@mdi/js';
import { computed, onMounted, ref, toRefs, watch } from 'vue';
import { VForm, VTextField } from 'vuetify/components';
import BlockchainAutocomplete from '~/components/inputs/BlockchainAutocomplete.vue';
import MetadataField from '~/components/inputs/MetadataField.vue';
import { AddressBookEntry, Asset } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { VFormValidation } from '~/types/helper.types';
import { compareMetadata, requiredRule } from '~/utils/form.utils';
import DiffView from '~/components/requests/DiffView.vue';

export type AddressBookFormProps = {
  modelValue: Partial<AddressBookEntry>;
  triggerSubmit?: boolean;
  valid?: boolean;
  mode?: 'view' | 'edit';
  currentEntry?: AddressBookEntry;
  display?: {
    id?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<AddressBookFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
  }),
  mode: 'edit',
  triggerSubmit: false,
  currentEntry: undefined,
});
const props = toRefs(input);

const isViewMode = computed(() => props.mode.value === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AddressBookFormProps['modelValue']): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: AddressBookFormProps['modelValue']): void;
}>();

const model = computed(() => props.modelValue.value);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const station = useStationStore();

const onSelectedBlockchain = (asset?: Asset): void => {
  if (asset) {
    model.value.blockchain = asset.blockchain;
    model.value.labels = [];
  } else {
    model.value.blockchain = undefined;
    model.value.labels = undefined;
  }
};

onMounted(() => {
  if (station.configuration.details.supported_assets.length === 1 && !model.value.blockchain) {
    onSelectedBlockchain(station.configuration.details.supported_assets[0]);
  }

  if (!model.value.metadata) {
    model.value.metadata = [];
  }
});

const isFormValid = computed(() => (form.value ? form.value.isValid : false));
watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

watch(
  () => props.triggerSubmit.value,
  () => {
    if (props.triggerSubmit.value) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};

const displayId = computed(() => props.display.value.id);
</script>
