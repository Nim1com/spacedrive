import { useBridgeMutation } from '@sd/client';
import { useCurrentLibrary } from '@sd/client';
import { Button, Form, Input, Select, SelectOption, Switch } from '@sd/ui';
import { useState } from 'react';
import { useForm } from 'react-hook-form';

import { InputContainer } from '../../../components/primitive/InputContainer';
import { SettingsContainer } from '../../../components/settings/SettingsContainer';
import { SettingsHeader } from '../../../components/settings/SettingsHeader';
import { useDebouncedForm } from '../../../hooks/useDebouncedForm';

export default function LibraryGeneralSettings() {
	const { library } = useCurrentLibrary();
	const { mutate: editLibrary } = useBridgeMutation('library.edit');
	const form = useForm({
		defaultValues: { id: library!.uuid, ...library?.config, todo: 'one' }
	});

	// useDebouncedForm(form, (value) =>
	// 	editLibrary({
	// 		id: library!.uuid,
	// 		name: value.name,
	// 		description: value.description
	// 	})
	// );

	form.watch((values) => {
		console.log(values);
	});

	const [todo, setTodo] = useState('');
	const [todo2, setTodo2] = useState('two');

	console.log(todo, todo2);

	return (
		<SettingsContainer>
			<Form
				form={form}
				onSubmit={(value) => {
					console.log(value); // TODO: force form submit
				}}
			>
				<h1>Controlled Inputs</h1>
				<Input {...form.register('name')} />
				<Select name="todo" control={form.control} className="mt-2">
					<SelectOption value="one">Option ONe</SelectOption>
					<SelectOption value="two">Option Two</SelectOption>
				</Select>
				{/* <Switch /> */}
				{/* <SubmitButton /> */}
			</Form>

			<h1>Uncontrolled Inputs</h1>
			<Input value={todo} onChange={(v) => setTodo(v)} />
			<Select value={todo2} onChange={(v) => setTodo2(v)}>
				<SelectOption value="one">Option One</SelectOption>
				<SelectOption value="two">Option Two</SelectOption>
			</Select>
			{/* <Switch /> */}
			{/* <SubmitButton /> */}

			{/* BREAK */}

			{/* <SettingsHeader
				title="Library Settings"
				description="General settings related to the currently active library."
			/>
			<div className="flex flex-row pb-3 space-x-5">
				<div className="flex flex-col flex-grow">
					<span className="mb-1 text-sm font-medium">Name</span>
					<Input {...form.register('name', { required: true })} defaultValue="My Default Library" />
				</div>
				<div className="flex flex-col flex-grow">
					<span className="mb-1 text-sm font-medium">Description</span>
					<Input {...form.register('description')} placeholder="" />
				</div>
			</div>

			<InputContainer
				mini
				title="Encrypt Library"
				description="Enable encryption for this library, this will only encrypt the Spacedrive database, not the files themselves."
			>
				<div className="flex items-center ml-3">
					<Switch checked={false} />
				</div>
			</InputContainer>
			<InputContainer mini title="Export Library" description="Export this library to a file.">
				<div className="mt-2">
					<Button size="sm" variant="gray">
						Export
					</Button>
				</div>
			</InputContainer>
			<InputContainer
				mini
				title="Delete Library"
				description="This is permanent, your files will not be deleted, only the Spacedrive library."
			>
				<div className="mt-2">
					<Button size="sm" variant="colored" className="bg-red-500 border-red-500">
						Delete
					</Button>
				</div>
			</InputContainer> */}
		</SettingsContainer>
	);
}
